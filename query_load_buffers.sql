SELECT
  id,
  role,
  name,
  (SELECT COUNT(*) FROM message WHERE message.buffer_id = buffer.id) AS message_count
FROM buffer
WHERE network_id = 0;
