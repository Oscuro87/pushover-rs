use super::{Message, PushoverSound};

// TODO: Fix DRY principle with attachment_message_builder.rs

/**
Helps build a correct Pushover request.
 */
#[derive(Debug)]
pub struct MessageBuilder {
    build: Message,
}

#[allow(dead_code)]
impl MessageBuilder {
    /// Creates a new MessageBuilder instance with the required minimal informations (User key, App token & Message)
    pub fn new(user_key: &str, application_token: &str, message: &str) -> Self {
        let mut build = Message::default();
        
        build.user_key = user_key.to_owned();
        build.app_token = application_token.to_owned();
        build.message = message.to_owned();

        MessageBuilder {
            build,
        }
    }

    /// Modifies the existing message.
    pub fn modify_message(mut self, message: &str) -> MessageBuilder {
        if message.trim().len() == 0 {
            return self;
        }

        self.build.message = message.to_owned();
        self
    }

    /// Sets a title to your message
    pub fn set_title(mut self, title: &str) -> MessageBuilder {
        if title.trim().len() == 0 {
            self.build.title = None;
        }

        self.build.title = Some(title.to_owned());
        self
    }

    /// Adds a title to your message
    #[deprecated(since="0.3.12", note="Please use set_title instead.")]
    pub fn add_title(mut self, title: &str) -> MessageBuilder {
        if title.trim().len() == 0 {
            self.build.title = None;
        }

        self.build.title = Some(title.to_owned());
        self
    }

    /// Removes the title. The title will be defaulted to your application name.
    pub fn remove_title(mut self) -> MessageBuilder {
        self.build.title = None;
        self
    }

    /// Sets an url (and optionally, an url title) to send along with your message.
    ///
    /// If set, the URL title will be shown, otherwise the URL will be shown.
    pub fn set_url(mut self, url: &str, url_title: Option<&str>) -> MessageBuilder {
        if url.trim().len() == 0 {
            self.build.url = None;
            self.build.url_title = None;
            return self;
        }

        self.build.url = Some(url.to_owned());
        if url_title.is_some() {
            self.build.url_title = Some(url_title.unwrap().to_owned());
        }
        self
    }

    /// Adds an url (and optionally, an url title) to send along with your message.
    /// 
    /// If set, the URL title will be shown, otherwise the URL will be shown.
    #[deprecated(since="0.3.12", note="Please use set_url instead.")]
    pub fn add_url(mut self, url: &str, url_title: Option<&str>) -> MessageBuilder {
        if url.trim().len() == 0 {
            self.build.url = None;
            self.build.url_title = None;
            return self;
        }

        self.build.url = Some(url.to_owned());
        if url_title.is_some() {
            self.build.url_title = Some(url_title.unwrap().to_owned());
        }
        self
    }

    /// Removes both the url and url title from your message
    pub fn remove_url(mut self) -> MessageBuilder {
        self.build.url = None;
        self.build.url_title = None;
        self
    }

    /// Send as -2 to generate no notification/alert, -1 to always send as a quiet notification, 1 to display as high-priority and bypass the user's quiet hours, or 2 to also require confirmation from the user.
    pub fn set_priority(mut self, priority: i8) -> MessageBuilder {
        if priority < -2 || priority > 2 {
            self.build.priority = Some(0);
            return self;
        }

        self.build.priority = Some(priority);
        self
    }

    /// Resets the priority to default (0, normal)
    pub fn remove_priority(mut self) -> MessageBuilder {
        self.build.priority = Some(0);
        self
    }

    /// Sets the sound to be used to notify the user.
    /// 
    /// See this list of available sounds: https://pushover.net/api#sounds
    pub fn set_sound(mut self, sound: PushoverSound) -> MessageBuilder {
        self.build.sound = Some(sound.to_string());
        self
    }

    /// Removes the custom sound and reverts to the default sound.
    pub fn remove_sound(mut self) -> MessageBuilder {
        self.build.sound = None;
        self
    }

    /// Sets an Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
    pub fn set_timestamp(mut self, unix_timestamp: u64) -> MessageBuilder {
        self.build.timestamp = Some(unix_timestamp);
        self
    }

    /// Resets the custom unix timestamp
    pub fn remove_timestamp(mut self) -> MessageBuilder {
        self.build.timestamp = None;
        self
    }

    /// Add a device name to send the notification to.
    /// 
    /// Ignores if the device name is already in the list.
    pub fn add_device(mut self, device_name: &str) -> MessageBuilder {
        type Devices = Vec<String>;

        if device_name.trim().len() == 0 {
            return self;
        }

        if self.build.devices.is_none() {
            self.build.devices = Some(vec!());
        } else {
            let clone: Vec<String> = self.build.devices.clone().unwrap();
            if clone.contains(&device_name.into()) {
                return self;
            }
        }

        let mut replacement_list: Devices = self.build.devices.clone().unwrap();
        replacement_list.push(device_name.to_owned());
        self.build.devices = Some(replacement_list);

        self
    }

    /// Overrides the current devices list with device_names
    pub fn set_devices(mut self, device_names: Vec<&str>) -> MessageBuilder {
        let device_names = device_names.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        self.build.devices = Some(device_names);
        self
    }

    /// Merges the current devices list with device_names, duplicates are eliminated
    pub fn merge_devices(mut self, device_names: Vec<&str>) -> MessageBuilder {
        for name in device_names {
            self = self.add_device(name);
        }
        self
    }

    /// Clears the devices list entirely
    pub fn clear_devices_list(mut self) -> MessageBuilder {
        self.build.devices = None;
        self
    }

    /// Transforms the MessageBuilder into a useable Message
    pub fn build(self) -> Message {
        self.build.clone()
    }
}
