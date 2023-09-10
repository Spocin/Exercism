export class Robot {
  private static usedNamesSet = new Set<string>();

  private _name: string;

  constructor() {
    this._name = this.findRandomName();
  }

  public get name(): string {
    return this._name;
  }

  public resetName(): void {
    this._name = this.findRandomName();
  }

  public static releaseNames(): void {
    Robot.usedNamesSet.clear();
  }

  private findRandomName(): string {
    let newName: string;

    do {
      newName = this.getRandomName();
    } while (Robot.usedNamesSet.has(newName))

    Robot.usedNamesSet.add(newName);

    return newName;
  }

  private getRandomName(): string {
    const opt: Intl.NumberFormatOptions = {
      minimumIntegerDigits: 3,
      useGrouping: false,
    }

    const firstChar = this.getRandomNumFromRange('A', 'Z');
    const secondChar = this.getRandomNumFromRange('A', 'Z');
    const num = Math.floor(Math.random() * 999);

    return `${firstChar}${secondChar}${num.toLocaleString('pl-PL', opt)}`;
  }

  private getRandomNumFromRange(min: string, max: string) {
    const minAsNum = min.charCodeAt(0);
    const maxAsNum = max.charCodeAt(0);
    const randomNum = Math.floor(Math.random() * (maxAsNum + 1 - minAsNum) + minAsNum);
    return String.fromCharCode(randomNum);
  }
}