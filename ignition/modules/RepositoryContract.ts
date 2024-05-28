import {buildModule} from "@nomicfoundation/hardhat-ignition/modules";

const LockModule = buildModule("RepositoryContractModule", (m) => {
    const lock = m.contract("RepositoryContract", [], {});

    return {lock};
});

export default LockModule;
