package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
)

type PostgresSettings struct {
	User     string
	Password string
	Database string
	Host     string
	Port     uint16
}

func (pgSettings *PostgresSettings) RunDocker() error {
	cmd := exec.Command(
		"docker", "run",
		"-e", "POSTGRES_USER="+pgSettings.User,
		"-e", "POSTGRES_PASSWORD="+pgSettings.Password,
		"-e", "POSTGRES_DB="+pgSettings.Database,
		"-p", fmt.Sprintf("%d:5432", pgSettings.Port),
		"-d", "postgres",
		"postgres", "-N", "1000",
	)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

var PgSettings = PostgresSettings{}

func init() {
	flag.StringVar(&PgSettings.User, "user", "axum", "Postgres user")
	flag.StringVar(&PgSettings.Password, "password", "axum", "Postgres password")
	flag.StringVar(&PgSettings.Database, "database", "axum", "Postgres database")
	flag.StringVar(&PgSettings.Host, "host", "localhost", "Postgres host")
	port := flag.Int("port", 5432, "Postgres port")
	PgSettings.Port = uint16(*port)
	flag.Parse()
}

func main() {
	if err := PgSettings.RunDocker(); err != nil {
		log.Fatal(err)
	}
}
