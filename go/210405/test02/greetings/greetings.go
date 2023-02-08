package greetings

import (
    "fmt"
    "errors"
    "math/rand"
    "time"
)

// Hello returns a greeting for the named person.
func Hello(name string) ( string, error ) {

    // If no name was given, return an error with a message.
    if name == "" {
        return "", errors.New("empty name")
    }

    // Return a greeting that embeds the name in a message.
    // message := fmt.Sprintf("Hi, %v. Welcome!", name)
    message := fmt.Sprintf(randomFormat(), name)
    // message := fmt.Sprint(randomFormat())
    return message, nil
}

func Hellos(names []string) (map[string]string, error) {
    messages := make(map[string]string)

    for _, name := range names {
        message, err := Hello(name)
        if err != nil {
            return nil, err
        }

        messages[name] = message
    }

    return messages, nil
}

func init() {
    rand.Seed(time.Now().UnixNano())
}

func randomFormat() string {
    formats := []string{
        "Hi, %v.",
        "Hi, %v. Welcome!",
        "Hello, %v.",
        "Hello, %v. Nice to meet you!",
    }

    return formats[rand.Intn(len(formats))]
}