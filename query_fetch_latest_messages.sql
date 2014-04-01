SELECT
  id, time_ns, type,
  information.message,
  'join'.who,
  privmsg.who, privmsg.message
FROM message
LEFT JOIN message_information information ON information.message_id = message.id
LEFT JOIN message_join 'join' ON 'join'.message_id = message.id
LEFT JOIN message_privmsg privmsg ON information.message_id = message.id
WHERE buffer_id = ?
SORT BY id DESC
LIMIT ?;
