//! Container for storing a list of resources with matching identifier type.

pub trait ResourceList<ResourceType: ?Sized, ResourceId>
{
	fn get(&self, index: ResourceId) -> Option<*const ResourceType>;
	fn count() -> ResourceId;
}
