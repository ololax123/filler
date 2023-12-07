import subprocess
import threading
import argparse

def run_game(command):
    # Split the command string into a list and then execute it
    process = subprocess.Popen(command.split(), stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout, stderr = process.communicate()

    # Check the output for the win status
    global player1_wins, player2_wins
    if "Player1 won" in stdout:
        player1_wins += 1
    elif "Player2 won" in stdout:
        player2_wins += 1

def main():
    global player1_wins, player2_wins
    player1_wins = 0
    player2_wins = 0

    # Parse command line arguments
    parser = argparse.ArgumentParser(description='Run a game command multiple times and track wins.')
    parser.add_argument('iterations', type=int, help='Number of times to run the command')
    parser.add_argument('command', type=str, help='The command to run as a single string')
    args = parser.parse_args()

    threads = []
    for i in range(args.iterations):
        t = threading.Thread(target=run_game, args=(args.command,))
        t.start()
        threads.append(t)

    for t in threads:
        t.join()

    # Display the statistics
    print("Player 1 won {} times.".format(player1_wins))
    print("Player 2 won {} times.".format(player2_wins))
    print("That's a {:.2f}% win rate for Player 1.".format(player1_wins / float(args.iterations) * 100))
    print("   And a {:.2f}% win rate for Player 2.".format(player2_wins / float(args.iterations) * 100))

if __name__ == "__main__":
    main()
