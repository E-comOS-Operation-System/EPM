/**
    EPM - E-comOS Packages Manager
    Copyright (C) 2025  E-comOS User Mode Team EPM Group & Saladin5101

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use serde::Deserialize;
use std::fs;
use anyhow::{Result, anyhow};

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub release_date: String,
    pub build_date: String,
    pub publisher: String,
    #[serde(rename = "is it verified?")]
    pub verified: String,
    pub author: String,
    pub run_on: String,
    pub description: String,
    pub license: String,
    #[serde(rename = "use_packages(If not used other packages, please write NOTHING)")]
    pub use_packages: String,
    #[serde(rename = "dependencies(If not dependent on other apps, please write No)")]
    pub dependencies: String,
    pub contact: serde_json::Value,
}

impl PackageMetadata {
    pub fn validate(&self) -> Result<()> {
        let required_fields = [
            (&self.name, "name"),
            (&self.version, "version"),
            (&self.publisher, "publisher"),
            (&self.author, "author"),
            (&self.description, "description"),
            (&self.license, "license"),
        ];
        
        for (field, field_name) in required_fields {
            if field.trim().is_empty() {  // Add trim() to check ' '
                anyhow::bail!("The field '{}' cannot be empty.", field_name);  // fix bail! use
            }
        }
        
        if !self.version.chars().any(|c| c == '.') {
            anyhow::bail!("The version field must contain a dot.");
        }
        
        let valid_statuses = ["yes", "true", "verified", "no", "false", "unverified"];
        if !valid_statuses.contains(&self.verified.to_lowercase().as_str()) {
            anyhow::bail!("Unknown status! The verified field must be one of: yes, true, verified, no, false, unverified.");
        }
        
        if self.dependencies != "No" && !self.dependencies.starts_with("epm://") {
            anyhow::bail!("The dependencies's format is an error, must be 'No' or start with 'epm://'");
        }
        
        if self.use_packages != "NOTHING" && !self.use_packages.starts_with("epm://") {
            anyhow::bail!("The use_packages's format is an error, must be 'NOTHING' or start with 'epm://'");
        }
        
        let valid_platforms = ["x86", "x64", "ARM", "ARM64", "RISC-V"];
        let platforms: Vec<&str> = self.run_on.split(',').map(|s| s.trim()).collect();
        
        for platform in &platforms {
            if !valid_platforms.contains(platform) {
                anyhow::bail!("Unsupported platform: '{}'. Supported platforms: {}", platform, valid_platforms.join(", "));
            }
        }
        
        println!("Package successfully verified"); 
        Ok(())
    }
    
    // Get bool to verify
    pub fn is_verified(&self) -> bool {
        matches!(self.verified.to_lowercase().as_str(), "yes" | "true" | "verified")
    }
    
    // Parse runtime platform as vector
    pub fn supported_platforms(&self) -> Vec<String> {
        self.run_on.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    // Check if depends on other packages
    pub fn has_dependencies(&self) -> bool {
        self.dependencies != "No"
    }
    
    // Get dependencies list
    pub fn dependency_list(&self) -> Vec<String> {
        if self.has_dependencies() {
            self.dependencies.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    // A from file head
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let metadata: PackageMetadata = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse metadata: {}", e))?;
        Ok(metadata)
    }
}