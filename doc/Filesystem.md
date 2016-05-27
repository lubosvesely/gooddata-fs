# GoodData Filesystem Format

## i-node ID format

i-node ID has 64 bits and is used as follow

* 16b - Project ID
* 8b  - Category ID
* 32b - Item ID
* 8b - Reserved

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
| 16  | metadata/datasets            |
| 17  | metadata/datefiltersettings  |
| 18  | metadata/dimensions          |
| 19  | metadata/domains             |
| 20  | metadata/etlfiles            |
| 21  | metadata/executioncontexts   |
| 22  | metadata/facts               |
| 23  | metadata/filters             |
| 24  | metadata/folders             |
| 25  | metadata/kpi                 |
| 26  | metadata/kpialert            |
| 27  | metadata/listattributefilter |
| 28  | metadata/metrics             |
| 29  | metadata/projectdashboards   |
| 30  | metadata/prompts             |
| 31  | metadata/reportdefinition    |
| 32  | metadata/reports             |
| 33  | metadata/scheduledmails      |
| 34  | metadata/tabledataloads      |
| 35  | metadata/tables              |
| 36  | metadata/userfilters         |
| 37  | metadata/visualizations      |
| 38  | publicartifacts              |
| 39  | roles                        |
| 40  | schedules                    |
| 41  | templates                    |
| 42  | uploads                      |
| 43  | users                        |

## Files

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

```
# tree --dirsfirst .

.
├── projects
│   ├── MyProject
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
