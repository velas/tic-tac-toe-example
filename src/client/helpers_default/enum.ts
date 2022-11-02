export default class Enum {
    enum: string | undefined;

    constructor(properties: any) {
	if (Object.keys(properties).length !== 1) {
	    throw new Error('Enum can only take single value');
	}
	Object.keys(properties).map((key) => {
	    this[key as keyof typeof this] = properties[key];
	    this.enum = key;
	});
    }
}
