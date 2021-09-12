package hss

import (
	"log"

	"go.ligato.io/cn-infra/v2/agent"
)

func StartAgent() {

	// Plugin instance
	hssPlugin := new(Agent)

	// Create new HSS front-end agent
	hssAgent := agent.NewAgent(agent.Plugins(hssPlugin))

	// Start the agent
	if err := hssAgent.Run(); err != nil {
		log.Fatalln(err)
	}

}

type Agent struct{}

// Ligato HSS plugin name
func (h *Agent) String() string {
	return "HssAgent"
}

// Initialize HSS agent
func (h *Agent) Init() error {
	log.Println("Initializing HSS agent")
	return nil
}

// Start HSS server after plugin initialization
func (h *Agent) AfterInit() error {
	log.Println("Starting HSS")
	Start()
	return nil
}

// On HSS shutdown
func (h *Agent) Close() error {
	log.Println("Shutting down HSS")
	Stop()
	return nil
}
