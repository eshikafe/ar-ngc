// Copyright (c) 2021 Austin Aigbe

package mme

import (
	"log"

	"go.ligato.io/cn-infra/v2/agent"
)

func StartAgent() {

	// Plugin instance
	mmePlugin := new(Agent)

	// Create new MME agent
	mmeAgent := agent.NewAgent(agent.Plugins(mmePlugin))

	// Start the agent
	if err := mmeAgent.Run(); err != nil {
		log.Fatalln(err)
	}

}

type Agent struct{}

// Ligato MME plugin name
func (h *Agent) String() string {
	return "MME-Agent"
}

// Initialize MME agent
func (h *Agent) Init() error {
	log.Println("Initializing MME agent")
	return nil
}

// Start MME after plugin initialization
func (h *Agent) AfterInit() error {
	log.Println("Starting ngc MME")
	Start()
	return nil
}

// On MME shutdown
func (h *Agent) Close() error {
	log.Println("Shutting down MME")
	Stop()
	return nil
}
