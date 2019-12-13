const crypto = require('crypto');

function calculateRandomNumberHash(randomNumber, timestamp) {
	console.log('randomNumber ' + randomNumber);
    console.log('timestamp ' + timestamp.toString());

	const timestampHexStr = timestamp.toString(16);
	var timestampHexStrFormat = timestampHexStr;

	// timestampHexStrFormat should be the hex string of a 32-length byte array. 
	// Fill 0 if the timestampHexStr length is less than 64
	for (var i = 0; i < 16 - timestampHexStr.length; i++) {
		timestampHexStrFormat = '0' + timestampHexStrFormat;
	}

	const timestampBytes = Buffer.from(timestampHexStrFormat, "hex");
	const newBuffer = Buffer.concat([Buffer.from(randomNumber.substring(2, 66), "hex"), timestampBytes]);
	const hash = crypto.createHash('sha256');
	hash.update(newBuffer);
	return "0x" + hash.digest('hex');
}

function calculateSwapID(randomNumberHash, receiver) {
	console.log('receiver ' + receiver.toString());
	
	const newBuffer = Buffer.concat([Buffer.from(randomNumberHash.substring(2, 66), "hex"), Buffer.from(receiver)]);
	const hash = crypto.createHash('sha256');
	hash.update(newBuffer);
	return "0x" + hash.digest('hex');
}

const run = async () => {
	let randomNumber = "0xaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccdd";

	// counted by second
    const timestamp = Math.floor(Date.now() / 1000); 
	
	let randomNumberHash = calculateRandomNumberHash(randomNumber, timestamp);
	console.log('randomNumberHash ' + randomNumberHash.toString('hex'));

	let receiver = "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL";

	let id = calculateSwapID(randomNumberHash, receiver);
	console.log('swapID ' + id.toString('hex'));
}

run();
