export class Clock {
  private DAY_AS_MINUTES = 1440;
  private minutes: number;

  constructor(hour: number, minute?: number) {
    let tmpMinutes =  ((hour * 60) + (minute ?? 0));

    if (tmpMinutes < 0) {
      tmpMinutes = tmpMinutes % this.DAY_AS_MINUTES;
      tmpMinutes = this.DAY_AS_MINUTES - (tmpMinutes * -1);
    }

    if (tmpMinutes >= this.DAY_AS_MINUTES) tmpMinutes = tmpMinutes % this.DAY_AS_MINUTES;
    this.minutes = tmpMinutes;
  }

  public toString(): string {
    const opt: Intl.NumberFormatOptions = {
      minimumIntegerDigits: 2,
      useGrouping: false,
    }

    const hours = Math.trunc(this.minutes / 60).toLocaleString('pl-PL', opt);
    const minutes = (this.minutes % 60).toLocaleString('pl-PL', opt);

    return `${hours}:${minutes}`
  }

  public plus(minutes: unknown): Clock {
    throw new Error('Remove this statement and implement this function')
  }

  public minus(minutes: unknown): Clock {
    throw new Error('Remove this statement and implement this function')
  }

  public equals(other: unknown): unknown {
    throw new Error('Remove this statement and implement this function')
  }
}
