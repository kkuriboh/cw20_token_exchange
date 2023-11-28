export const bail = (message: string): never => {
  console.error(message);
  return process.exit(1);
};
