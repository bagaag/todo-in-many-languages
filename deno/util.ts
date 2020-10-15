// returns true if the given path exists
export const pathExists = (filename: string): boolean => {
    try {
      Deno.statSync(filename);
      return true;
    } catch (error) {
      if (error && error instanceof Deno.errors.NotFound) {
        return false;
      } else {
        throw error;
      }
    }
};