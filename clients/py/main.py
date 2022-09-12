import os
from dotenv import load_dotenv
from web3 import Web3
import json

load_dotenv()

print(os.environ["SIMPLE_STORAGE"])


def main():
    web3 = Web3(Web3.HTTPProvider('http://127.0.0.1:8545'))
    contract_address = os.environ['SIMPLE_STORAGE']
    abi = json.load(open("../abis/SimpleStorage.json"))['abi']

    SimpleStorage = web3.eth.contract(address=contract_address, abi=abi)
    valueBefore = SimpleStorage.functions.get().call()
    print(valueBefore)
    SimpleStorage.functions.set(valueBefore + 123).transact()
    valueAfter = SimpleStorage.functions.get().call()
    print(valueAfter)


if __name__ == "__main__":
    main()
