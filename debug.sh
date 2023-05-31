#!/bin/bash

# Output to stdout:
# cargo run -- -p resources/aic -i agent_wsdl.xml

# Output to file:
cargo run -- -p resources/aic -i agent_wsdl.xml -o examples/aic/aic_agent.rs
cargo run -- -p resources/aic -i version_wsdl.xml -o examples/aic/aic_version.rs
cargo run -- -p resources/aic -i workflow_wsdl.xml -o examples/aic/aic_workflow.rs
cargo run -- -p resources/hello -i hello.wsdl -o examples/hello/src/hello.rs
cargo run -- -p resources/temp_converter -i tempconverter.wsdl -o examples/temperature/tempconverter.rs
cargo run -- -p resources/weather -i weather.wsdl -o examples/weather/weather.rs
cargo run -- -p resources/smgr -i userimport.xsd -o examples/smgr/smgr.rs -d "http://xml.avaya.com/schema/import"
cargo run -- -p resources/smgr -i agentCommProfile.xsd -o examples/smgr/smgr_agent.rs -n agent -d "http://xml.avaya.com/schema/import"
cargo run -- -p resources/smgr -i stationCommProfile.xsd -o examples/smgr/smgr_station.rs -n csm -d "http://xml.avaya.com/schema/import"
cargo run -- -p resources/smgr -i presence.xsd -o examples/smgr/smgr_presence.rs -n ps -d "http://xml.avaya.com/schema/import"
cargo run -- -p resources/smgr -i SessionManager.xsd -o examples/smgr/smgr_sm.rs -n asm -d "http://xml.avaya.com/schema/import"
cargo run -- -p resources/smgr -i officelinxProfile.xsd -o examples/smgr/smgr_officelinx.rs -n ol -d "http://xml.avaya.com/schema/import"
cargo run -- -p resources/smgr -i userdeltaimport.xsd -o examples/smgr/smgr_delta_import.rs -n delta -d "http://xml.avaya.com/schema/deltaImport"
cargo run -- -p resources/smgr -i UserProfileSchemaDefinitionForBulkDelete.xsd -o examples/smgr/smgr_delete.rs -d "http://xml.avaya.com/schema/bulkdelete"
cargo run -- -p resources/smgr -i "User Management Lookup_criteria.xsd" -o examples/smgr/smgr_lookup_criteria.rs -d "http://www.avaya.com/mgmt-web-criteria/"

# Format the output
rustfmt --color=always --edition=2018 examples/aic/aic_agent.rs
rustfmt --color=always --edition=2018 examples/aic/aic_version.rs
rustfmt --color=always --edition=2018 examples/aic/aic_workflow.rs
rustfmt --color=always --edition=2018 examples/hello/src/hello.rs
rustfmt --color=always --edition=2018 examples/temperature/tempconverter.rs
rustfmt --color=always --edition=2018 examples/weather/weather.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_station.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_agent.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_presence.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_sm.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_officelinx.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_delta_import.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_delete.rs
rustfmt --color=always --edition=2018 examples/smgr/smgr_lookup_criteria.rs