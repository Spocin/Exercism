export class Robot {
  private static readonly namesArr: string[] = [];

  private _name: string;
  private usedNamesSet = new Set<string>();

  private static initializeNamesMap() {
    const opt: Intl.NumberFormatOptions = {
      minimumIntegerDigits: 3,
      useGrouping: false,
    }

    for (let i = 65; i < 91; i++) {
      for (let j = 65; j < 91; j++) {
        for (let k = 0; k <= 999; k++) {
          Robot.namesArr.push(
              `${String.fromCharCode(i)}${String.fromCharCode(j)}${k.toLocaleString('pl-PL', opt)}`
          )
        }
      }
    }
  }

  constructor() {
    if (!Robot.namesArr.length) Robot.initializeNamesMap();
    this._name = this.getRandomName();
    this.usedNamesSet.add(this._name);
  }

  public get name(): string {
    return this._name;
  }

  public resetName(): void {
    let newName = this.getRandomName();

    while (this.usedNamesSet.has(newName)) {
      Robot.namesArr.push(newName);
      newName = this.getRandomName();
    }

    this._name = newName;
  }

  public static releaseNames(): void {
    Robot.namesArr.length = 0;
  }

  private getRandomName(): string {
    const chosenIdx = Math.floor(Math.random() * Robot.namesArr.length);
    const choseName = Robot.namesArr[chosenIdx];
    Robot.namesArr.splice(chosenIdx, 1);
    return choseName;
  }
}