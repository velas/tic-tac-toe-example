export default class Struct {
    constructor(properties: any) {
        Object.keys(properties).map((key) => {
            this[key as keyof typeof this] = properties[key];
        });
    }
}
