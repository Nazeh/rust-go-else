const HEX =
  '49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d';

const EXPECTED =
  'SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t';

const hexToBase64 = (hex) => {
  let buf = Buffer.from(hex, 'hex');
  return buf.toString('base64');
};

const result = hexToBase64(HEX);

module.exports = () => {
  result === EXPECTED
    ? console.log('SUCESS at Set 1, challenge 1')
    : console.log(
        'FAIL at Set 1, challenge 1\n',
        `Expected ${EXPECTED}\nGot${result}`,
      );
};
