package hss

import (
	"log"

	"go.ligato.io/cn-infra/v2/agent"
)

func StartHssAgent() {

	// Plugin instance
	hss_plugin := new(HssAgent)

	// Create new HSS front-end agent
	hss_agent := agent.NewAgent(agent.Plugins(hss_plugin))

	// Start the agent
	if err := hss_agent.Run(); err != nil {
		log.Fatalln(err)
	}

}

type HssAgent struct{}

// Ligato HSS plugin name
func (h *HssAgent) String() string {
	return "HssAgent"
}

// Initialize HSS agent
func (h *HssAgent) Init() error {
	log.Println("Initializing HSS agent")
	return nil
}

// Start HSS server after plugin initialization
func (h *HssAgent) AfterInit() error {
	log.Println("Starting HSS")
	Start()
	return nil
}

// On HSS shutdown
func (h *HssAgent) Close() error {
	log.Println("Shutting down HSS agent")
	Stop()
	return nil
}
