export class Robot {
  private static namesMap = new Map<string, boolean>();

  private static initializeNamesMap() {
    const opt: Intl.NumberFormatOptions = {
      minimumIntegerDigits: 3,
      useGrouping: false,
    }

    for (let i = 65; i < 91; i++) {
      for (let j = 65; j < 91; j++) {
        for (let k = 0; k <= 999; k++) {
          this.namesMap.set(
              `${String.fromCharCode(i)}${String.fromCharCode(j)}${k.toLocaleString('pl-PL', opt)}`,
              false,
          )
        }
      }
    }
  }

  constructor() {
    if (!Robot.namesMap.size) Robot.initializeNamesMap();
  }

  public get name(): string {
    throw new Error('Implement Robot#name')
  }

  public resetName(): void {
    throw new Error('Implement Robot#resetName')
  }

  public static releaseNames(): void {
    throw new Error('Implement Robot.releaseNames')
  }
}