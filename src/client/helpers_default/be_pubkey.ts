import { PublicKey } from '@velas/web3';

export default class PublicKeyBE {
    solPubKey: PublicKey;
    value: Uint8Array;

    constructor( fields: { value : Uint8Array } ) {
	this.value = fields.value;
	let copy = new Uint8Array(fields.value);
	this.solPubKey = new PublicKey(copy);
    }

    fromPubKey(arg: PublicKey) : this {
	this.solPubKey = arg;
	let copy = new Uint8Array(Uint8Array.from(arg.toBuffer()))
	this.value = copy;

	return this
    }

    equals(publicKey: PublicKeyBE) {
	return this.solPubKey.equals(publicKey.solPubKey);
    }

}
