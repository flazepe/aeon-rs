pub static DNS_CODES: [[&str; 2]; 10] = [
    ["NOERROR", "DNS Query completed successfully."],
    ["FORMERR", "DNS Query Format Error."],
    ["SERVFAIL", "Server failed to complete the DNS request."],
    ["NXDOMAIN", "Domain name does not exist."],
    ["NOTIMP", "Function not implemented."],
    ["REFUSED", "The server refused to answer for the query."],
    ["YXDOMAIN", "Name that should not exist, does exist."],
    ["XRRSET", "RRset that should not exist, does exist."],
    ["NOTAUTH", "Server not authoritative for the zone."],
    ["NOTZONE", "Name not in zone."],
];
