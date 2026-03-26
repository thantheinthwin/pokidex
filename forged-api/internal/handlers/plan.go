package handlers

import "github.com/gin-gonic/gin"

type PlanHandler struct{}

func NewPlanHandler() *PlanHandler {
	return &PlanHandler{}
}

func (h *PlanHandler) Generate(c *gin.Context) {
	respondOK(c)
}
