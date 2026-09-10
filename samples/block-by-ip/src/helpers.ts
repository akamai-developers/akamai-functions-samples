const getClientAddressFromRequest = (req: Request): string | null => {
  const clientAddress = req.headers.get("spin-client-addr");
  if (clientAddress) {
    return clientAddress;
  }
  return req.headers.get("true-client-ip");
};

const cleanupIpAddress = (input: string): string => {
  const ipv4Regex = /^(\d{1,3}\.){3}\d{1,3}:/;
  const ipv6WithPortRegex = /^\[([a-fA-F0-9:]+)\]:\d+$/;

  if (ipv4Regex.test(input)) {
    return input.split(':')[0];
  } else if (ipv6WithPortRegex.test(input)) {
    return input.match(ipv6WithPortRegex)![1];
  } else {
    return input;
  }
};

export {
  getClientAddressFromRequest,
  cleanupIpAddress
};
