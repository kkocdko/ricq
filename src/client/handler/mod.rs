use async_trait::async_trait;
use tokio::sync::{
    broadcast::Sender as BroadcastSender,
    mpsc::{Sender as MpscSender, UnboundedSender},
    watch::Sender as WatchSender,
};

use crate::client::event::{
    FriendMessageRecallEvent, FriendRequestEvent, GroupMessageEvent, GroupMuteEvent,
    GroupRequestEvent, NewFriendEvent, NewMemberEvent, PrivateMessageEvent,
};

/// 所有需要外发的数据的枚举打包
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub enum QEvent {
    TcpConnect,
    TcpDisconnect,
    /// 登录成功事件
    LoginEvent(i64),
    /// 群消息
    GroupMessage(GroupMessageEvent),
    /// 群自身消息
    SelfGroupMessage(GroupMessageEvent),
    /// 私聊消息
    PrivateMessage(PrivateMessageEvent),
    /// 加群申请
    GroupRequest(GroupRequestEvent),
    /// 加好友申请
    FriendRequest(FriendRequestEvent),
    /// 新成员入群
    NewMember(NewMemberEvent),
    /// 成员被禁言
    GroupMute(GroupMuteEvent),
    /// 好友消息撤回
    FriendMessageRecall(FriendMessageRecallEvent),
    /// 新好友
    NewFriend(NewFriendEvent),
    // FriendList(decoder::friendlist::FriendListResponse),
    // GroupMemberInfo(structs::GroupMemberInfo),

    // 群消息发送成功事件 内部处理
    // GroupMessageReceipt(GroupMessageReceiptEvent)
}

/// 处理外发数据的接口
#[async_trait]
pub trait Handler: Sync {
    async fn handle(&self, msg: QEvent) {
        match msg {
            QEvent::LoginEvent(uin) => self.handle_login_event(uin).await,
            QEvent::GroupMessage(group_message) => self.handle_group_message(group_message).await,
            QEvent::SelfGroupMessage(group_message) => {
                self.handle_self_group_message(group_message).await
            }
            QEvent::PrivateMessage(private_message) => {
                self.handle_private_message(private_message).await
            }
            QEvent::GroupRequest(group_request) => self.handle_group_request(group_request).await,
            QEvent::FriendRequest(friend_request) => {
                self.handle_friend_request(friend_request).await
            }
            QEvent::NewMember(new_member) => self.handle_new_member(new_member).await,
            QEvent::GroupMute(group_mute) => self.handle_group_mute(group_mute).await,
            QEvent::NewFriend(new_friend) => self.handle_new_friend(new_friend).await,
            QEvent::FriendMessageRecall(friend_message_recall) => {
                self.handle_friend_message_recall(friend_message_recall)
                    .await
            }
            QEvent::TcpConnect => self.handle_tcp_connect_event().await,
            QEvent::TcpDisconnect => self.handle_tcp_connect_event().await,
        }
    }
    async fn handle_login_event(&self, _uin: i64) {}
    async fn handle_tcp_connect_event(&self) {}
    async fn handle_tcp_disconnect_event(&self) {}
    async fn handle_group_message(&self, _group_message: GroupMessageEvent) {}
    async fn handle_self_group_message(&self, _group_message: GroupMessageEvent) {}
    async fn handle_private_message(&self, _private_message: PrivateMessageEvent) {}
    async fn handle_group_request(&self, _group_request: GroupRequestEvent) {}
    async fn handle_friend_request(&self, _group_request: FriendRequestEvent) {}
    async fn handle_new_member(&self, _new_member: NewMemberEvent) {}
    async fn handle_group_mute(&self, _group_mute: GroupMuteEvent) {}
    async fn handle_friend_message_recall(&self, _friend_message_recall: FriendMessageRecallEvent) {
    }
    async fn handle_new_friend(&self, _new_friend: NewFriendEvent) {}
}

/// 一个默认 Handler，只是把信息打印出来
pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {
    async fn handle(&self, e: QEvent) {
        match e {
            QEvent::GroupMessage(m) => {
                println!(
                    "MESSAGE (GROUP={}): {}",
                    m.message.group_code, m.message.elements
                )
            }
            QEvent::PrivateMessage(m) => {
                println!(
                    "MESSAGE (FRIEND={}): {}",
                    m.message.from_uin, m.message.elements
                )
            }
            QEvent::GroupRequest(m) => {
                println!(
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.request.group_code, m.request.req_uin, m.request.message
                )
            }
            QEvent::FriendRequest(m) => {
                println!("REQUEST (UIN={}): {}", m.request.req_uin, m.request.message)
            }
            _ => println!("{:?}", e),
        }
    }
}

#[async_trait]
impl Handler for BroadcastSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler for MpscSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).await.unwrap();
    }
}

#[async_trait]
impl Handler for UnboundedSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler for WatchSender<QEvent> {
    async fn handle(&self, msg: QEvent) {
        self.send(msg).unwrap();
    }
}
