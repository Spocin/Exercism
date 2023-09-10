export class Clock {
  private readonly DAY_AS_MINUTES = 1440;
  private readonly minutes: number;

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

  public plus(minutes: number): Clock {
    return new Clock(0, this.minutes + minutes);
  }

  public minus(minutes: number): Clock {
    return new Clock(0, this.minutes - minutes);
  }

  public equals(other: Clock): boolean {
    return this.minutes === other.minutes;
  }
}
