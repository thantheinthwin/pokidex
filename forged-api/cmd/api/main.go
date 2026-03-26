package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gin-gonic/gin"

	"forged-api/internal/handlers"
	"forged-api/internal/middleware"
)

type config struct {
	AppKey           string
	BetterAuthSecret string
	Port             string
	AppVersion       string
	ShutdownTimeout  time.Duration
}

func loadConfig() config {
	return config{
		AppKey:           getEnv("APP_KEY", "dev-app-key"),
		BetterAuthSecret: getEnv("BETTER_AUTH_SECRET", "dev-better-auth-secret"),
		Port:             getEnv("PORT", "8080"),
		AppVersion:       getEnv("APP_VERSION", "dev"),
		ShutdownTimeout:  getEnvDuration("SHUTDOWN_TIMEOUT", 10*time.Second),
	}
}

func getEnv(key, fallback string) string {
	if v, ok := os.LookupEnv(key); ok && v != "" {
		return v
	}
	return fallback
}

func getEnvDuration(key string, fallback time.Duration) time.Duration {
	v, ok := os.LookupEnv(key)
	if !ok || v == "" {
		return fallback
	}
	d, err := time.ParseDuration(v)
	if err != nil {
		log.Printf("invalid duration for %s: %v; using fallback %s", key, err, fallback)
		return fallback
	}
	return d
}

func main() {
	cfg := loadConfig()

	r := gin.New()
	r.GET("/health", handlers.Health(cfg.AppVersion))

	v1 := r.Group("/v1")
	v1.Use(
		middleware.AppKeyMiddleware(cfg.AppKey),
		middleware.AuthMiddleware(cfg.BetterAuthSecret),
		middleware.LoggingMiddleware(),
		middleware.RecoveryMiddleware(),
	)
	{
		workoutHandler := handlers.NewWorkoutHandler()
		coachHandler := handlers.NewCoachHandler()
		planHandler := handlers.NewPlanHandler()
		analyzeHandler := handlers.NewAnalyzeHandler()

		workouts := v1.Group("/workouts")
		{
			workouts.POST("/log", workoutHandler.Log)
			workouts.GET("/history", workoutHandler.GetHistory)
		}

		coach := v1.Group("/coach")
		coach.POST("/chat", coachHandler.Chat)

		plans := v1.Group("/plans")
		plans.POST("/generate", planHandler.Generate)

		analyze := v1.Group("/analyze")
		analyze.POST("/body-comp", analyzeHandler.BodyComp)
	}

	srv := &http.Server{Addr: ":" + cfg.Port, Handler: r}

	go func() {
		log.Printf("starting API server on %s", srv.Addr)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("server failed: %v", err)
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	ctx, cancel := context.WithTimeout(context.Background(), cfg.ShutdownTimeout)
	defer cancel()

	log.Printf("shutting down server with timeout %s", cfg.ShutdownTimeout)
	if err := srv.Shutdown(ctx); err != nil {
		log.Printf("graceful shutdown failed: %v", err)
		if closeErr := srv.Close(); closeErr != nil {
			log.Printf("forced server close failed: %v", closeErr)
		}
	}
}
