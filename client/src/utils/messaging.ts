import { ChatMessageT, ChatMessage, ChatSource } from "@fb/Chat";
import { Builder, ByteBuffer } from "flatbuffers";

export function deserializeChatMessage(bytes: Uint8Array): ChatMessageT {
  let dataBuffer = new ByteBuffer(bytes);
  return ChatMessage.getRootAsChatMessage(dataBuffer).unpack();
}

export function serializeChatMessage(
  message: string,
  author: string,
  source: ChatSource,
): Uint8Array {
  const builder = new Builder();

  const messageOffset = builder.createString(message);
  const authorOffset = builder.createString(author);
  const chatMessage = ChatMessage.createChatMessage(builder, source, messageOffset, authorOffset);
  builder.finish(chatMessage);

  return builder.asUint8Array();
}
