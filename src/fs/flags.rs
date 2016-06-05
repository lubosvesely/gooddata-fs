#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Category {
    Internal,
    Connectors,
    Dataload,
    DataloadDownload,
    DataloadEventstore,
    DataloadMetadataStorage,
    DataloadProcesses,
    EventStores,
    Invitations,
    Ldm,
    Metadata,
    MetadataAnalyticDashboard,
    MetadataAttributes,
    MetadataColumns,
    MetadataDataLoadingColumns,
    MetadataDatasets,
    MetadataDateFilterSettings,
    MetadataDimensions,
    MetadataDomains,
    MetadataEtlFiles,
    MetadataExecutionContexts,
    MetadataFacts,
    MetadataFilters,
    MetadataFolders,
    MetadataKpi,
    MetadataKpiAlert,
    MetadataListAttributeFilter,
    MetadataMetrics,
    MetadataProjectDashboards,
    MetadataPrompts,
    MetadataReportDefinition,
    MetadataReports,
    MetadataSchedulEdmails,
    MetadataTableDataLoads,
    MetadataTables,
    MetadataUserFilters,
    MetadataVisualizations,
    PublicArtifacts,
    Roles,
    Schedules,
    Templates,
    Uploads,
    Users,
}

#[derive(Debug, Clone)]
pub enum ReservedFile {
    Root = 0,
    FeatureFlagsJson = 2,
    PermissionsJson,
    ProjectJson,
    RolesJson,

    // Keep me - see https://github.com/osxfuse/osxfuse/issues/286
    KeepMe = 255,
}

impl From<u8> for ReservedFile {
    fn from(val: u8) -> ReservedFile {
        match val {
            0 => ReservedFile::Root,
            2 => ReservedFile::FeatureFlagsJson,
            3 => ReservedFile::PermissionsJson,
            4 => ReservedFile::ProjectJson,
            5 => ReservedFile::RolesJson,
            _ => ReservedFile::KeepMe,
        }
    }
}
