use crate::{
    discord::discord_connection::ConnectionInner,
    twilight_utils::{ext::MemberExt, Color, GroupIdExt},
};
use std::rc::Rc;
use twilight_model::{gateway::payload::MemberListItem, id::GuildId};
use weechat::buffer::{BufferHandle, NickSettings};

pub struct Nicklist {
    conn: ConnectionInner,
    guild_id: Option<GuildId>,
    handle: Rc<BufferHandle>,
}

impl Nicklist {
    pub fn new(
        conn: &ConnectionInner,
        guild_id: Option<GuildId>,
        handle: Rc<BufferHandle>,
    ) -> Nicklist {
        Nicklist {
            conn: conn.clone(),
            guild_id,
            handle,
        }
    }

    pub fn update(&self, member_list: &[MemberListItem]) {
        if let Ok(buffer) = self.handle.upgrade() {
            // TODO: Optimize with diffing/change tracking?
            buffer.clear_nicklist();

            let mut current_group_idx = 0;
            let mut current_group = None;

            for item in member_list {
                match item {
                    MemberListItem::Group(group) => {
                        let role_color = group
                            .id
                            .role(&self.conn.cache)
                            .map(|role| role.color)
                            .filter(|&c| c != 0)
                            .map(|c| Color::new(c).as_8bit().to_string())
                            .unwrap_or_else(|| "default".to_owned());
                        let nick_group = buffer
                            .add_nicklist_group(
                                &format!(
                                    "{}|{}",
                                    current_group_idx,
                                    group.id.name(&self.conn.cache)
                                ),
                                &role_color,
                                true,
                                None,
                            )
                            .unwrap();
                        current_group = Some(nick_group);
                        current_group_idx += 1;
                        continue;
                    },
                    MemberListItem::Member(member) => {
                        let nick_group = if let Some(nick_group) = current_group.as_ref() {
                            nick_group
                        } else {
                            tracing::error!("Nick list in an invalid state: {:#?}", member_list);
                            continue;
                        };
                        if let Some(guild_id) = self.guild_id {
                            if let Some(guild_member) =
                                self.conn.cache.member(guild_id, member.user.id)
                            {
                                let color = guild_member
                                    .color(&self.conn.cache)
                                    .filter(|&c| c.0 != 0)
                                    .map(|c| c.as_8bit().to_string());

                                let mut settings = NickSettings::new(&guild_member.display_name());
                                if let Some(ref color) = color {
                                    settings = settings.set_color(color);
                                }
                                if let Err(()) = nick_group.add_nick(settings) {
                                    tracing::warn!(
                                        "Failed to add member \"{}\" to nicklist",
                                        member.user.username
                                    );
                                }
                            }
                        } else if let Err(()) =
                            nick_group.add_nick(NickSettings::new(&member.user.username))
                        {
                            tracing::warn!(
                                "Failed to add member \"{}\" to nicklist",
                                member.user.username
                            );
                        }
                    },
                };
            }
        }
    }
}
