use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedData<T: Debug, C: Debug> {
    #[serde(flatten)]
    pub data: T,
    #[serde(rename = "@context")]
    pub context: C,
}

pub trait AsLinkedData: Sized + Debug {
    type ContextType: Debug;

    fn get_ld_context(&self) -> Self::ContextType;

    fn into_linked_data(self) -> LinkedData<Self, Self::ContextType> {
        LinkedData {
            context: self.get_ld_context(),
            data: self,
        }
    }
}

pub mod activity_streams {
    use serde::{Deserialize, Serialize};
    use crate::linkeddata::{marker_types};
    use crate::linkeddata::activity_streams::objects::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum MaybeObject {
        URI(marker_types::URI),
        TaggedLink(TaggedLink),
        TaggedObject(TaggedObject),
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum MaybeLink {
        URI(marker_types::URI),
        TaggedLink(TaggedLink),
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum TaggedLink {
        Link(Link)
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum TaggedObject {
        Object(Object),
        Activity(Activity),
        Collection(Collection),
        OrderedCollection(OrderedCollection),
        CollectionPage(CollectionPage),
        OrderedCollectionPage(OrderedCollectionPage),
        // Extended
        // Activity types
        Accept(Accept),
        TentativeAccept(TentativeAccept),
        Add(Add),
        Arrive(Arrive),
        Create(Create),
        Delete(Delete),
        Follow(Follow),
        Ignore(Ignore),
        Join(Join),
        Leave(Leave),
        Like(Like),
        Offer(Offer),
        Invite(Invite),
        Reject(Reject),
        TentativeReject(TentativeReject),
        Remove(Remove),
        Undo(Undo),
        Update(Update),
        View(View),
        Listen(Listen),
        Read(Read),
        Move(Move),
        Travel(Travel),
        Announce(Announce),
        Block(Block),
        Flag(Flag),
        Dislike(Dislike),
        Question(Question),
        // Actor types
        Application(Application),
        Group(Group),
        Organization(Organization),
        Person(Person),
        Service(Service),
        // Object types
        Relationship(Relationship),
        Article(Article),
        Document(Document),
        Audio(Audio),
        Image(Image),
        Video(Video),
        Note(Note),
        Page(Page),
        Event(Event),
        Place(Place),
        Mention(Mention),
        Profile(Profile),
        Tombstone(Tombstone),
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum MaybeImage {
        URI(marker_types::URI),
        TaggedLink(TaggedLink),
        TaggedImage(TaggedImage),
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum TaggedImage {
        Image(Image)
    }

    #[allow(non_snake_case)]    // These structs are serialized; The names of their fields map directly to those in the emitted/received JSON
    mod objects {
        use serde::{Deserialize, Serialize};
        use crate::linkeddata::activity_streams::properties::*;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Object {
            attachment: Option<PropAttachment>,
            attributedTo: Option<PropAttributedTo>,
            audience: Option<PropAudience>,
            content: Option<PropContent>,
            contentMap: Option<PropContentMap>,
            context: Option<PropContext>,
            name: Option<PropName>,
            nameMap: Option<PropNameMap>,
            endTime: Option<PropEndTime>,
            generator: Option<PropGenerator>,
            icon: Option<PropIcon>,
            image: Option<PropImage>,
            inReplyTo: Option<PropInReplyTo>,
            location: Option<PropLocation>,
            preview: Option<PropPreview>,
            published: Option<PropPublished>,
            replies: Option<PropReplies>,
            startTime: Option<PropStartTime>,
            summary: Option<PropSummary>,
            summaryMap: Option<PropSummaryMap>,
            tag: Option<PropTag>,
            updated: Option<PropUpdated>,
            url: Option<PropUrl>,
            to: Option<PropTo>,
            bto: Option<PropBTO>,
            cc: Option<PropCC>,
            bcc: Option<PropBCC>,
            mediaType: Option<PropMediaType>,
            duration: Option<PropDuration>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Link {
            href: Option<PropHref>,
            rel: Option<PropRel>,
            mediaType: Option<PropMediaType>,
            name: Option<PropName>,
            nameMap: Option<PropNameMap>,
            hreflang: Option<PropHrefLang>,
            height: Option<PropHeight>,
            width: Option<PropWidth>,
            preview: Option<PropPreview>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Activity {
            actor: Option<PropActor>,
            object: Option<PropObject>,
            target: Option<PropTarget>,
            result: Option<PropResult>,
            origin: Option<PropOrigin>,
            instrument: Option<PropInstrument>,
            #[serde(flatten)]
            object_fields: Object,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct IntransitiveActivity(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Collection {
            totalItems: Option<PropTotalItems>,
            current: Option<PropCurrent>,
            first: Option<PropFirst>,
            last: Option<PropLast>,
            items: Option<PropItems>,
            #[serde(flatten)]
            object_fields: Object,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OrderedCollection {
            totalItems: Option<PropTotalItems>,
            current: Option<PropCurrent>,
            first: Option<PropFirst>,
            last: Option<PropLast>,
            items: Option<PropItems>,
            orderedItems: Option<PropItems>,
            #[serde(flatten)]
            object_fields: Object,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CollectionPage {
            partOf: Option<PropPartOf>,
            next: Option<PropNext>,
            prev: Option<PropPrev>,
            #[serde(flatten)]
            collection_fields: Collection,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OrderedCollectionPage {
            startIndex: Option<PropStartIndex>,
            partOf: Option<PropPartOf>,
            next: Option<PropNext>,
            prev: Option<PropPrev>,
            #[serde(flatten)]
            collection_fields: OrderedCollection,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Accept(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TentativeAccept(Activity);   // Inherits from accept, but this would result in another level of indirection, so inherit Activity directly

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Add(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Arrive(Activity);    // Inherits from IntransitiveActivity

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Create(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Delete(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Follow(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Ignore(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Join(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Leave(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Like(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Offer(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Invite(Activity);    // Inherits from Offer

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Reject(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TentativeReject(Activity);   // Inherits from Reject

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Remove(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Undo(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Update(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct View(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Listen(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Read(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Move(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Travel(Activity);    // Inherits from IntransitiveActivity

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Announce(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Block(Activity);     // Inherits from Ignore

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Flag(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Dislike(Activity);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Question {
            oneOf: Option<PropOneOf>,
            anyOf: Option<PropAnyOf>,
            closed: Option<PropClosed>,
            #[serde(flatten)]
            activity_fields: Activity,   // Inherits from IntransitiveActivity
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Application(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Group(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Organization(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Person(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Service(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Relationship {
            subject: Option<PropSubject>,
            object: Option<PropObject>,
            relationship: Option<PropRelationship>,
            #[serde(flatten)]
            object_fields: Object,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Article(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Document(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Audio(Object);    // Inherits from Document

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Image(Object);    // Inherits from Document

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Video(Object);    // Inherits from Document

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Note(Object);    // Inherits from Document

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Page(Object);    // Inherits from Document

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Event(Object);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Place {
            accuracy: Option<PropAccuracy>,
            altitude: Option<PropAltitude>,
            latitude: Option<PropLatitude>,
            longitude: Option<PropLongitude>,
            radius: Option<PropRadius>,
            units: Option<PropUnits>,
            #[serde(flatten)]
            object_fields: Object,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Mention(Link);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Profile {
            describes: Option<PropDescribes>,
            #[serde(flatten)]
            object_fields: Object,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Tombstone {
            formerType: Option<PropFormerType>,
            deleted: Option<PropDeleted>,
            #[serde(flatten)]
            object_fields: Object,
        }
    }

    mod properties {
        use serde::{Deserialize, Serialize};
        use chrono::{DateTime, FixedOffset};
        use crate::linkeddata::activity_streams::{MaybeImage, MaybeLink, MaybeObject, TaggedLink, TaggedObject};
        use crate::linkeddata::activity_streams::objects::*;
        use crate::linkeddata::util::FoldedSlice;
        use crate::linkeddata::marker_types;
        use crate::linkeddata::marker_types::{Duration, LocalizedString, RadiusAltitudeUnit, RFC5988};

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum MaybeClosed {
            URI(marker_types::URI),
            TaggedLink(TaggedLink),
            TaggedObject(TaggedObject),
            Date(DateTime<FixedOffset>),
            Bool(bool),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum MaybeCollection {
            URI(marker_types::URI),
            TaggedLink(TaggedLink),
            TaggedCollection(TaggedCollection),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag = "type")]
        pub enum TaggedCollection {
            Collection(Collection),
            OrderedCollection(OrderedCollection),
            // TODO: Maybe not pages?
            CollectionPage(CollectionPage),
            OrderedCollectionPage(OrderedCollectionPage),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum MaybeCollectionPage {
            URI(marker_types::URI),
            TaggedLink(TaggedLink),
            Tagged(TaggedCollectionPage),
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag = "type")]
        pub enum TaggedCollectionPage {
            CollectionPage(CollectionPage),
            OrderedCollectionPage(OrderedCollectionPage),
        }

        #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
        #[serde(try_from = "f64")]
        pub struct Percentage {
            value: f64,
        }

        impl Percentage {
            pub fn get(self) -> f64 {
                self.value
            }
        }

        impl TryFrom<f64> for Percentage {
            type Error = &'static str;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                match value {
                    value if value >= 0.0 && value <= 100.0 => Ok(Percentage { value }),
                    _ => Err("Percentage must be +0 <= X <= +100")
                }
            }
        }

        #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
        #[serde(try_from = "f64")]
        pub struct PositiveFloat {
            value: f64,
        }

        impl PositiveFloat {
            pub fn get(self) -> f64 {
                self.value
            }
        }

        impl TryFrom<f64> for PositiveFloat {
            type Error = &'static str;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                match value {
                    value if value.is_sign_positive() => Ok(PositiveFloat { value }),
                    _ => Err("Positive float must be >= +0")
                }
            }
        }

        // ID and Type attribute are handled by LinkedData struct and ActivityStreamType Enum Variants respectively
        pub type PropActor = FoldedSlice<MaybeObject>;
        pub type PropAttachment = FoldedSlice<MaybeObject>;
        pub type PropAttributedTo = FoldedSlice<MaybeObject>;
        pub type PropAudience = FoldedSlice<MaybeObject>;
        pub type PropBCC = FoldedSlice<MaybeObject>;
        pub type PropBTO = FoldedSlice<MaybeObject>;
        pub type PropCC = FoldedSlice<MaybeObject>;
        pub type PropContext = FoldedSlice<MaybeObject>;
        pub type PropCurrent = Box<MaybeCollectionPage>;
        pub type PropFirst = Box<MaybeCollectionPage>;
        pub type PropGenerator = FoldedSlice<MaybeObject>;
        pub type PropIcon = FoldedSlice<MaybeImage>;
        pub type PropImage = FoldedSlice<MaybeImage>;
        pub type PropInReplyTo = FoldedSlice<MaybeObject>;
        pub type PropInstrument = FoldedSlice<MaybeObject>;
        pub type PropLast = Box<MaybeCollectionPage>;
        pub type PropLocation = FoldedSlice<MaybeObject>;
        pub type PropItems = FoldedSlice<MaybeObject>;
        pub type PropOneOf = FoldedSlice<MaybeObject>;
        pub type PropAnyOf = FoldedSlice<MaybeObject>;
        pub type PropClosed = Box<MaybeClosed>;
        pub type PropOrigin = FoldedSlice<MaybeObject>;
        pub type PropNext = Box<MaybeCollectionPage>;
        pub type PropObject = FoldedSlice<MaybeObject>;
        pub type PropPrev = Box<MaybeCollectionPage>;
        pub type PropPreview = FoldedSlice<MaybeObject>;
        pub type PropResult = FoldedSlice<MaybeObject>;
        pub type PropReplies = Box<MaybeCollection>;
        pub type PropTag = FoldedSlice<MaybeObject>;
        pub type PropTarget = FoldedSlice<MaybeObject>;
        pub type PropTo = FoldedSlice<MaybeObject>;
        pub type PropUrl = FoldedSlice<MaybeLink>;
        pub type PropAccuracy = Percentage;
        pub type PropAltitude = f64;
        pub type PropContent = String;
        pub type PropContentMap = LocalizedString;
        pub type PropName = String;
        pub type PropNameMap = LocalizedString;
        pub type PropDuration = Duration;
        pub type PropHeight = u64;
        pub type PropHref = marker_types::URI;
        pub type PropHrefLang = marker_types::LanguageTag;
        pub type PropPartOf = Box<MaybeCollection>;
        pub type PropLatitude = f64;
        pub type PropLongitude = f64;
        pub type PropMediaType = marker_types::MIMEType;
        pub type PropEndTime = DateTime<FixedOffset>;
        pub type PropPublished = DateTime<FixedOffset>;
        pub type PropStartTime = DateTime<FixedOffset>;
        pub type PropRadius = PositiveFloat;
        pub type PropRel = FoldedSlice<RFC5988>;
        pub type PropStartIndex = u64;
        pub type PropSummary = String;
        pub type PropSummaryMap = LocalizedString;
        pub type PropTotalItems = u64;
        pub type PropUnits = RadiusAltitudeUnit;
        pub type PropUpdated = DateTime<FixedOffset>;
        pub type PropWidth = u64;
        pub type PropSubject = Box<MaybeObject>;
        pub type PropRelationship = FoldedSlice<TaggedObject>;
        pub type PropDescribes = Box<TaggedObject>;
        pub type PropFormerType = FoldedSlice<TaggedObject>;
        pub type PropDeleted = DateTime<FixedOffset>;
    }
}

mod marker_types {
    use std::collections::HashMap;

    pub type URI = String;
    pub type LanguageTag = String;
    pub type MIMEType = String;
    pub type RFC5988 = String;
    pub type RadiusAltitudeUnit = String;
    pub type Duration = String;
    pub type LocalizedString = HashMap<String, String>;
}

pub mod util {
    use std::ops::Deref;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum FoldedSlice<T> {
        One(Box<T>),
        Many(Box<[T]>),
    }

    impl<T> From<T> for FoldedSlice<T> {
        fn from(value: T) -> Self {
            FoldedSlice::One(Box::new(value))
        }
    }

    impl<T> From<Vec<T>> for FoldedSlice<T> {
        fn from(many: Vec<T>) -> Self {
            if many.len() == 1 {
                // This re-allocates
                let value = many.into_iter().next().unwrap();
                FoldedSlice::One(Box::new(value))
            } else {
                FoldedSlice::Many(many.into_boxed_slice())
            }
            // Reallocation-free implementation
            // match <Box<[T; 1]>>::try_from(many.into_boxed_slice()) {
            //     Ok(single) => FoldedSlice::One(unsafe { Box::from_raw(Box::into_raw(single).cast::<T>()) }),
            //     Err(many) => FoldedSlice::Many(many)
            // }
        }
    }

    impl<T> Deref for FoldedSlice<T> {
        type Target = [T];

        fn deref(&self) -> &Self::Target {
            match self {
                FoldedSlice::One(one) => std::slice::from_ref(&**one),
                FoldedSlice::Many(many) => &**many
            }
        }
    }
}