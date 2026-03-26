package middleware

import (
	"log"
	"time"

	"github.com/gin-gonic/gin"
)

func LoggingMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		start := time.Now()
		c.Next()

		latency := time.Since(start)
		log.Printf(
			"method=%s path=%s status=%d latency=%s request_id=%q platform=%q client_version=%q",
			c.Request.Method,
			c.Request.URL.Path,
			c.Writer.Status(),
			latency,
			c.GetHeader("X-Request-ID"),
			c.GetHeader("X-Platform"),
			c.GetHeader("X-Client-Version"),
		)
	}
}
