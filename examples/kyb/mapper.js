
const handler = (input) => {

  const count = (flag) => Object.values(input?.flag || {}).reduce((acc, curr) => {
    if (curr === flag) return acc + 1;
    return acc;
  }, 0);

  return {
    critical: count('critical'),
    red: count('red'),
    amber: count('amber'),
    green: count('green')
  };
}
