package handlers

import "github.com/gin-gonic/gin"

type WorkoutHandler struct{}

func NewWorkoutHandler() *WorkoutHandler {
	return &WorkoutHandler{}
}

func (h *WorkoutHandler) Log(c *gin.Context) {
	respondOK(c)
}

func (h *WorkoutHandler) GetHistory(c *gin.Context) {
	respondOK(c)
}
