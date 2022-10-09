export class TodoError extends Error {
  constructor(m?: string) {
    super(m);
    this.name = 'TodoError';
    if (m === undefined) {
      this.message = 'This is unimpletemented feature. ';
    }
  }
}
