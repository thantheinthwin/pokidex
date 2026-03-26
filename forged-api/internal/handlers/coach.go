package handlers

import "github.com/gin-gonic/gin"

type CoachHandler struct{}

func NewCoachHandler() *CoachHandler {
	return &CoachHandler{}
}

func (h *CoachHandler) Chat(c *gin.Context) {
	respondOK(c)
}
