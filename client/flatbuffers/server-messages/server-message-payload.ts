// automatically generated by the FlatBuffers compiler, do not modify

import { ChatPayload } from '../server-messages/chat-payload';
import { JoinPayload } from '../server-messages/join-payload';
import { LeavePayload } from '../server-messages/leave-payload';
import { StatePayload } from '../server-messages/state-payload';


export enum ServerMessagePayload {
  NONE = 0,
  JoinPayload = 1,
  LeavePayload = 2,
  ChatPayload = 3,
  StatePayload = 4
}

export function unionToServerMessagePayload(
  type: ServerMessagePayload,
  accessor: (obj:ChatPayload|JoinPayload|LeavePayload|StatePayload) => ChatPayload|JoinPayload|LeavePayload|StatePayload|null
): ChatPayload|JoinPayload|LeavePayload|StatePayload|null {
  switch(ServerMessagePayload[type]) {
    case 'NONE': return null; 
    case 'JoinPayload': return accessor(new JoinPayload())! as JoinPayload;
    case 'LeavePayload': return accessor(new LeavePayload())! as LeavePayload;
    case 'ChatPayload': return accessor(new ChatPayload())! as ChatPayload;
    case 'StatePayload': return accessor(new StatePayload())! as StatePayload;
    default: return null;
  }
}

export function unionListToServerMessagePayload(
  type: ServerMessagePayload, 
  accessor: (index: number, obj:ChatPayload|JoinPayload|LeavePayload|StatePayload) => ChatPayload|JoinPayload|LeavePayload|StatePayload|null, 
  index: number
): ChatPayload|JoinPayload|LeavePayload|StatePayload|null {
  switch(ServerMessagePayload[type]) {
    case 'NONE': return null; 
    case 'JoinPayload': return accessor(index, new JoinPayload())! as JoinPayload;
    case 'LeavePayload': return accessor(index, new LeavePayload())! as LeavePayload;
    case 'ChatPayload': return accessor(index, new ChatPayload())! as ChatPayload;
    case 'StatePayload': return accessor(index, new StatePayload())! as StatePayload;
    default: return null;
  }
}