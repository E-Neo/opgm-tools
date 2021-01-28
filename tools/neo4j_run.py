import time
import argparse
from neo4j import GraphDatabase
from neo4j.exceptions import ServiceUnavailable


def build_match(cypher_file):
    with open(cypher_file, 'r') as f:
        cypher = f.read()

    def match(tx):
        count = 0
        for _ in tx.run(cypher):
            count += 1
        return count

    return match


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Cypher executor')
    parser.add_argument('CYPHER_FILE')
    parser.add_argument('--uri', default='neo4j://localhost:7687')
    parser.add_argument('--user', default='neo4j')
    parser.add_argument('--password', default='neo4j')
    args = parser.parse_args()

    while True:
        try:
            driver = GraphDatabase.driver(
                args.uri, auth=(args.user, args.password))
            with driver.session() as session:
                time_now = time.time_ns()
                num_rows = session.read_transaction(
                    build_match(args.CYPHER_FILE))
                print('num_rows:', num_rows)
                print('total_time:', (time.time_ns() - time_now) // 1000000)
                break
        except ServiceUnavailable:
            time.sleep(1)
