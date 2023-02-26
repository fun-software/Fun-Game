// automatically generated by the FlatBuffers compiler, do not modify

import { ChatPayload } from '../client-messages/chat-payload';
import { JoinPayload } from '../client-messages/join-payload';
import { LeavePayload } from '../client-messages/leave-payload';
import { MovePayload } from '../client-messages/move-payload';


export enum ClientMessagePayload {
  NONE = 0,
  JoinPayload = 1,
  LeavePayload = 2,
  ChatPayload = 3,
  MovePayload = 4
}

export function unionToClientMessagePayload(
  type: ClientMessagePayload,
  accessor: (obj:ChatPayload|JoinPayload|LeavePayload|MovePayload) => ChatPayload|JoinPayload|LeavePayload|MovePayload|null
): ChatPayload|JoinPayload|LeavePayload|MovePayload|null {
  switch(ClientMessagePayload[type]) {
    case 'NONE': return null; 
    case 'JoinPayload': return accessor(new JoinPayload())! as JoinPayload;
    case 'LeavePayload': return accessor(new LeavePayload())! as LeavePayload;
    case 'ChatPayload': return accessor(new ChatPayload())! as ChatPayload;
    case 'MovePayload': return accessor(new MovePayload())! as MovePayload;
    default: return null;
  }
}

export function unionListToClientMessagePayload(
  type: ClientMessagePayload, 
  accessor: (index: number, obj:ChatPayload|JoinPayload|LeavePayload|MovePayload) => ChatPayload|JoinPayload|LeavePayload|MovePayload|null, 
  index: number
): ChatPayload|JoinPayload|LeavePayload|MovePayload|null {
  switch(ClientMessagePayload[type]) {
    case 'NONE': return null; 
    case 'JoinPayload': return accessor(index, new JoinPayload())! as JoinPayload;
    case 'LeavePayload': return accessor(index, new LeavePayload())! as LeavePayload;
    case 'ChatPayload': return accessor(index, new ChatPayload())! as ChatPayload;
    case 'MovePayload': return accessor(index, new MovePayload())! as MovePayload;
    default: return null;
  }
}