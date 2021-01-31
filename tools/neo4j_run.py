import sys
import time
import argparse
from neo4j import GraphDatabase
from neo4j.exceptions import ServiceUnavailable


def build_match(cypher, to_stdout):
    def match(tx):
        count = 0
        for _ in tx.run(cypher):
            count += 1
        return count

    def to_stdout_match(tx):
        rtns = [x.strip()
                for x in cypher[cypher.index('RETURN') + 6:].split(',')]
        count = 0
        for row in tx.run(cypher):
            print(*[row[x] for x in rtns], sep=',')
            count += 1
        return count

    return to_stdout_match if to_stdout else match


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Cypher executor')
    parser.add_argument('CYPHER_FILE')
    parser.add_argument('--uri', default='neo4j://localhost:7687')
    parser.add_argument('--user', default='neo4j')
    parser.add_argument('--password', default='neo4j')
    parser.add_argument('--to-stdout', action='store_true')
    args = parser.parse_args()

    while True:
        try:
            driver = GraphDatabase.driver(
                args.uri, auth=(args.user, args.password))
            with driver.session() as session:
                with open(args.CYPHER_FILE, 'r') as f:
                    cypher = f.read()
                time_now = time.time_ns()
                num_rows = session.read_transaction(
                    build_match(cypher, args.to_stdout))
                print('num_rows:', num_rows, file=sys.stderr)
                print('total_time:', (time.time_ns() - time_now) //
                      1000000, file=sys.stderr)
                break
        except ServiceUnavailable:
            time.sleep(1)
