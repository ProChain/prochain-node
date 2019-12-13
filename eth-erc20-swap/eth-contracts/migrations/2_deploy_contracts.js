const PraToken = artifacts.require("ProToken");
const ERC20AtomicSwap = artifacts.require("ERC20HTLC");

module.exports = function(deployer) {
    deployer.deploy(PraToken, "10000000000000000", "PRA Token", "8", "PRA").then(function(){
        return deployer.deploy(ERC20AtomicSwap, PraToken.address);
    });
    //deployer.deploy(ETHAtomicSwap)
};
