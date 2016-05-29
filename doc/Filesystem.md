# GoodData Filesystem Format

## i-node ID format

i-node ID has 64 bits and is used as follow

| Bits | Description  |
|------|--------------|
| 16b  | Project ID   |
| 8b   | Category ID  |
| 32b  | Item ID      |
| 8b   | Reserved     |

## Category ID

| ID  | Usage                        |
|-----|------------------------------|
| 0   | internal                     |
| 1   | connectors                   |
| 2   | dataload                     |
| 3   | dataload/download            |
| 4   | dataload/eventstore          |
| 5   | dataload/metadatastorage     |
| 6   | dataload/processes           |
| 7   | eventstores                  |
| 8   | invitations                  |
| 9   | ldm                          |
| 10  | metadata                     |
| 11  | metadata/analyticdashboard   |
| 12  | metadata/attributes          |
| 13  | metadata/columns             |
| 14  | metadata/dataloadingcolumns  |
| 15  | metadata/datasets            |
| 16  | metadata/datefiltersettings  |
| 17  | metadata/dimensions          |
| 18  | metadata/domains             |
| 19  | metadata/etlfiles            |
| 20  | metadata/executioncontexts   |
| 21  | metadata/facts               |
| 22  | metadata/filters             |
| 23  | metadata/folders             |
| 24  | metadata/kpi                 |
| 25  | metadata/kpialert            |
| 26  | metadata/listattributefilter |
| 27  | metadata/metrics             |
| 28  | metadata/projectdashboards   |
| 29  | metadata/prompts             |
| 30  | metadata/reportdefinition    |
| 31  | metadata/reports             |
| 32  | metadata/scheduledmails      |
| 33  | metadata/tabledataloads      |
| 34  | metadata/tables              |
| 35  | metadata/userfilters         |
| 36  | metadata/visualizations      |
| 37  | publicartifacts              |
| 38  | roles                        |
| 39  | schedules                    |
| 40  | templates                    |
| 41  | uploads                      |
| 42  | users                        |

## Files - Category Type `internal`

| Offset | File              |
|--------|-------------------|
| 0      | .                 |
| 1      | ..                |
| 2      | featureflags.json |
| 3      | permissions.json  |
| 4      | project.json      |
| 5      | roles.json        |

## Reserved

| ID |  Usage     |
|----|------------|
| 1  | root       |
| 2  | /user.json |
| 3  | /projects  |

## Tree

### root

Items marked `✓` are already implemented.

```
# tree --dirsfirst .

.
├── projects ✓
│   ├── MyProject ✓
│   │   ├── connectors
│   │   ├── dataload
│   │   │   ├── download
│   │   │   │   ├── facebook.json
│   │   │   │   ├── facebooktokens.json
│   │   │   │   ├── googleanalytics.json
│   │   │   │   ├── salesforcedeletedrecords.json
│   │   │   │   └── salesforcedownload.json
│   │   │   ├── eventstore
│   │   │   ├── metadatastorage
│   │   │   └── processes
│   │   │       └── d8a081ff-074c-4891-9f1a-565ada7e4ab8.json
│   │   ├── eventstores
│   │   ├── invitations
│   │   ├── ldm
│   │   │   └── thumbnail.svg
│   │   ├── metadata
│   │   │   ├── analyticdashboard
│   │   │   ├── attributes
│   │   │   ├── columns
│   │   │   ├── dataloadingcolumns
│   │   │   ├── datasets
│   │   │   ├── datefiltersettings
│   │   │   ├── dimensions
│   │   │   ├── domains
│   │   │   ├── etlfiles
│   │   │   ├── executioncontexts
│   │   │   ├── facts
│   │   │   ├── filters
│   │   │   ├── folders
│   │   │   ├── kpi
│   │   │   ├── kpialert
│   │   │   ├── listattributefilter
│   │   │   ├── metrics
│   │   │   ├── projectdashboards
│   │   │   ├── prompts
│   │   │   ├── reportdefinition
│   │   │   ├── reports
│   │   │   ├── scheduledmails
│   │   │   ├── tabledataloads
│   │   │   ├── tables
│   │   │   ├── userfilters
│   │   │   └── visualizations
│   │   ├── publicartifacts
│   │   ├── roles
│   │   ├── schedules
│   │   ├── templates
│   │   ├── uploads
│   │   ├── users
│   │   ├── featureflags.json
│   │   ├── permissions.json
│   │   ├── project.json
│   │   └── roles.json
│   └── projects.json ✓
└── user.json ✓

44 directories, 13 files

```
