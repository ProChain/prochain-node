const PraToken = artifacts.require("ProToken");
const ERC20AtomicSwap = artifacts.require("ERC20HTLC");
const crypto = require('crypto');
const truffleAssert = require('truffle-assertions');

function calculateRandomNumberHash(randomNumber, timestamp) {
	const timestampHexStr = timestamp.toString(16);
	var timestampHexStrFormat = timestampHexStr;
	// timestampHexStrFormat should be the hex string of a 32-length byte array. Fill 0 if the timestampHexStr length is less than 64
	for (var i = 0; i < 16 - timestampHexStr.length; i++) {
		timestampHexStrFormat = '0' + timestampHexStrFormat;
	}
	const timestampBytes = Buffer.from(timestampHexStrFormat, "hex");
	const newBuffer = Buffer.concat([Buffer.from(randomNumber.substring(2, 66), "hex"), timestampBytes]);
	const hash = crypto.createHash('sha256');
	hash.update(newBuffer);
	return "0x" + hash.digest('hex');
}

function calculateSwapID(randomNumberHash, sender) {
	const newBuffer = Buffer.concat([Buffer.from(randomNumberHash.substring(2, 66), "hex"), Buffer.from(sender.substring(2, 42), "hex")]);
	const hash = crypto.createHash('sha256');
	hash.update(newBuffer);
	return "0x" + hash.digest('hex');
}

contract('Verify PRA Token and ERC20 Atomic Swap', (accounts) => {
	it('Check init state for PRA Token and ERC20 Atomic Swap', async () => {
		const initSupply = 10000000000000000;

		let instance = await PraToken.deployed();
		let balance = (await instance.balanceOf.call(accounts[0])).valueOf();
		assert.equal(Number(balance.toString()), initSupply, "10000000000000000 wasn't in the first account");

		const name = await instance.name.call();
		assert.equal(name, "PRA Token", "Contract name should be PRA Token");

		const symbol = await instance.symbol.call();
		assert.equal(symbol, "PRA", "Token symbol should be PRA");

		const decimals = await instance.decimals.call();
		assert.equal(decimals, 8, "Token decimals should be 8");

		const totalSupply = await instance.totalSupply.call();
		assert.equal(Number(totalSupply.toString()), initSupply, "Token total supply should be 10000000000000000");

		const swapInstance = await ERC20AtomicSwap.deployed();
		const praContractAddr = await swapInstance.PraContractAddr.call();
		assert.equal(praContractAddr, PraToken.address, "swap contract should have erc20 contract address");
	});
	it('Test transfer, approve and transferFrom for PRA token', async () => {
		const instance = await PraToken.deployed();
		const acc0 = accounts[0];
		const acc1 = accounts[1];
		const acc2 = accounts[2];
		const acc3 = accounts[3];
		const amount = 1000000000000;

		await instance.transfer(acc1, amount, { from: acc0 });
		const acc1Balance = (await instance.balanceOf.call(acc1)).valueOf();
		assert.equal(Number(acc1Balance.toString()), amount, "acc1 balance should be " + amount);

		await instance.approve(acc2, amount, { from: acc1 });
		await instance.transferFrom(acc1, acc3, amount, { from: acc2 });

		const balanceAcc1 = (await instance.balanceOf.call(acc1)).valueOf();
		const balanceAcc2 = (await instance.balanceOf.call(acc2)).valueOf();
		const balanceAcc3 = (await instance.balanceOf.call(acc3)).valueOf();

		assert.equal(Number(balanceAcc1.toString()), 0, "acc1 balance should be 0");
		assert.equal(Number(balanceAcc2.toString()), 0, "acc2 balance should be 0");
		assert.equal(Number(balanceAcc3.toString()), amount, "acc3 balance should be " + amount);

		await instance.approve(acc2, amount, { from: acc0 });
		await instance.transferFrom(acc0, acc2, amount, { from: acc2 });
		const balanceAcc2_1 = (await instance.balanceOf.call(acc2)).valueOf();
		assert.equal(Number(balanceAcc2_1.toString()), amount, "acc2 balance should be " + amount);
	});
	it('Test swap initiate and claim', async () => {
		const swapInstance = await ERC20AtomicSwap.deployed();
		const instance = await PraToken.deployed();

		const swapA = accounts[0];
		const swapB = accounts[4];

		const timestamp = Math.floor(Date.now() / 1000); // counted by second
		const randomNumber = "0xaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccdd";
		const randomNumberHash = calculateRandomNumberHash(randomNumber, timestamp);
		const heightSpan = 1000;
		const recipientAddr = swapB;
		const praDIDAddr = "did:pra:Lt23xGimVoUNvZ3EXM9FcgBsJXzrSaUo8p";
		const erc20Amount = 100000000;
		const praAmount = 100000000;
		const swapID = calculateSwapID(randomNumberHash, swapA);

		var isSwapExist = (await swapInstance.isSwapExist.call(randomNumberHash)).valueOf();
		assert.equal(isSwapExist, false);

		await instance.approve(ERC20AtomicSwap.address, erc20Amount, { from: swapA });
		let initiateTx = await swapInstance.htlc(randomNumberHash, timestamp, heightSpan, recipientAddr, erc20Amount, praAmount, praDIDAddr, { from: swapA });

		console.log("swapA:", swapA);
		console.log("swapB:", swapB);
		console.log("praDIDAddr:", praDIDAddr);
		console.log("swapID:", swapID);
		console.log("randomNumber:", randomNumber);
		console.log("randomNumberHash:", randomNumberHash);
		console.log("timestamp:", timestamp);
		console.log("erc20Amount:", erc20Amount);
		console.log("praAmount:", praAmount);

		//SwapInit event should be emitted
		truffleAssert.eventEmitted(initiateTx, 'HTLC', (ev) => {
			return ev._msgSender === swapA &&
				ev._recipientAddr === swapB &&
				//ev._receiverAddr === praDIDAddr &&
				ev._swapID === swapID &&
				ev._randomNumberHash === randomNumberHash &&
				Number(ev._timestamp.toString()) === timestamp &&
				Number(ev._outAmount.toString()) === erc20Amount &&
				Number(ev._praAmount.toString()) === praAmount;
		});

		console.log("initiateTx gasUsed: ", initiateTx.receipt.gasUsed);

		// Verify if the swapped ERC20 token has been transferred to contract address
		var balanceOfSwapContract = await instance.balanceOf.call(ERC20AtomicSwap.address);
		assert.equal(Number(balanceOfSwapContract.toString()), erc20Amount);

		// querySwapByHashLock
		var swap = (await swapInstance.queryOpenSwap.call(swapID)).valueOf();
		assert.equal(timestamp, swap._timestamp);
		assert.equal(swapA, swap._sender);

		isSwapExist = (await swapInstance.isSwapExist.call(swapID)).valueOf();
		assert.equal(isSwapExist, true);
		var claimable = (await swapInstance.claimable.call(swapID)).valueOf();
		assert.equal(claimable, true);
		var refundable = (await swapInstance.refundable.call(swapID)).valueOf();
		assert.equal(refundable, false);

		var balanceOfSwapB = await instance.balanceOf.call(swapB);
		assert.equal(Number(balanceOfSwapB.toString()), 0);

		// Anyone can call claim and the token will be paid to swapB address
		let claimTx = await swapInstance.claim(swapID, randomNumber, { from: accounts[6] });
		//SwapComplete and Claimed event should be emitted
		truffleAssert.eventEmitted(claimTx, 'Claimed', (ev) => {
			return ev._msgSender === accounts[6] &&
				ev._recipientAddr === swapB &&
				ev._swapID === swapID &&
				//ev._randomNumberHash === randomNumberHash &&
				ev._randomNumber === randomNumber;
		});
		console.log("claimTx gasUsed: ", claimTx.receipt.gasUsed);

		balanceOfSwapB = await instance.balanceOf.call(swapB);
		assert.equal(Number(balanceOfSwapB.toString()), erc20Amount);

		balanceOfSwapContract = await instance.balanceOf.call(ERC20AtomicSwap.address);
		assert.equal(Number(balanceOfSwapContract.toString()), 0);

		claimable = (await swapInstance.claimable.call(swapID)).valueOf();
		assert.equal(claimable, false);
		refundable = (await swapInstance.refundable.call(swapID)).valueOf();
		assert.equal(refundable, false);
	});
	it('Test swap initiate, refund', async () => {
		const swapInstance = await ERC20AtomicSwap.deployed();
		const instance = await PraToken.deployed();

		const swapA = accounts[0];
		const swapB = accounts[5];

		const timestamp = Math.floor(Date.now() / 1000); // counted by second
		const randomNumber = "0x5566778855667788556677885566778855667788556677885566778855667788";
		const randomNumberHash = calculateRandomNumberHash(randomNumber, timestamp);
		const heightSpan = 100;
		const recipientAddr = swapB;
		const praDIDAddr = "did:pra:Lt23xGimVoUNvZ3EXM9FcgBsJXzrSaUo8p";
		const erc20Amount = 100000000;
		const praAmount = 100000000;
		const swapID = calculateSwapID(randomNumberHash, swapA);

		var isSwapExist = (await swapInstance.isSwapExist.call(swapID)).valueOf();
		assert.equal(isSwapExist, false);

		await instance.approve(ERC20AtomicSwap.address, erc20Amount, { from: swapA });
		let initiateTx = await swapInstance.htlc(randomNumberHash, timestamp, heightSpan, recipientAddr, erc20Amount, praAmount, praDIDAddr, { from: swapA });
		//SwapInit event should be emitted
		truffleAssert.eventEmitted(initiateTx, 'HTLC', (ev) => {
			return ev._msgSender === swapA &&
				ev._recipientAddr === swapB &&
				//ev._receiverAddr === praDIDAddr &&
				ev._swapID === swapID &&
				ev._randomNumberHash === randomNumberHash &&
				Number(ev._timestamp.toString()) === timestamp &&
				Number(ev._outAmount.toString()) === erc20Amount &&
				Number(ev._praAmount.toString()) === praAmount;
		});
		console.log("initiateTx gasUsed: ", initiateTx.receipt.gasUsed);

		isSwapExist = (await swapInstance.isSwapExist.call(swapID)).valueOf();
		assert.equal(isSwapExist, true);
		var claimable = (await swapInstance.claimable.call(swapID)).valueOf();
		assert.equal(claimable, true);
		var refundable = (await swapInstance.refundable.call(swapID)).valueOf();
		assert.equal(refundable, false);

		// Just for producing new blocks
		for (var i = 0; i < heightSpan * 2; i++) {
			await instance.transfer(swapA, 10, { from: swapA });
		}

		claimable = (await swapInstance.claimable.call(swapID)).valueOf();
		assert.equal(claimable, false);
		refundable = (await swapInstance.refundable.call(swapID)).valueOf();
		assert.equal(refundable, true);

		var balanceOfSwapA = await instance.balanceOf.call(swapA);
		var balanceOfSwapB = await instance.balanceOf.call(swapB);
		assert.equal(Number(balanceOfSwapB.toString()), 0);

		// Anyone can call refund and the token will always been refunded to swapA address
		let refundTx = await swapInstance.refund(swapID, { from: accounts[6] });

		//SwapExpire and Refunded event should be emitted
		truffleAssert.eventEmitted(refundTx, 'Refunded', (ev) => {
			return ev._msgSender === accounts[6] &&
				ev._recipientAddr === swapA &&
				ev._swapID === swapID &&
				ev._randomNumberHash === randomNumberHash;
		});
		console.log("refundTx gasUsed: ", refundTx.receipt.gasUsed);

		balanceOfSwapB = await instance.balanceOf.call(swapB);
		assert.equal(Number(balanceOfSwapB.toString()), 0);

		var balanceOfSwapANew = await instance.balanceOf.call(swapA);
		assert.equal(Number(balanceOfSwapANew.toString()), Number(balanceOfSwapA.toString()) + erc20Amount);

		var balanceOfSwapContract = await instance.balanceOf.call(ERC20AtomicSwap.address);
		assert.equal(Number(balanceOfSwapContract.toString()), 0);

		claimable = (await swapInstance.claimable.call(swapID)).valueOf();
		assert.equal(claimable, false);
		refundable = (await swapInstance.refundable.call(swapID)).valueOf();
		assert.equal(refundable, false);
	});
});
