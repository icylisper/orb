
const handler = (input) => {

  let fees = input['fees']['percent'];

  return {
    verified: true,
    fees: fees
  };

}
