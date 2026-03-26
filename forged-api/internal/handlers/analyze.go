package handlers

import "github.com/gin-gonic/gin"

type AnalyzeHandler struct{}

func NewAnalyzeHandler() *AnalyzeHandler {
	return &AnalyzeHandler{}
}

func (h *AnalyzeHandler) BodyComp(c *gin.Context) {
	respondOK(c)
}
