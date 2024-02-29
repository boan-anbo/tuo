use std::collections::HashMap;

use async_trait::async_trait;

use crate::core::agency::agent::AgentTrait;
use crate::core::agency::profile::ProfileTrait;
use crate::core::agency::project::project::{Project, ProjectResult, Stage};
use crate::core::agency::project::project_director::ProjectDirectorTrait;
use crate::core::messaging::memory::Memory;
use crate::core::messaging::message::Message;
use crate::error::TuoError;
use crate::model::model::CompletionModelTrait;

/// Project assistant trait
///
/// A project assistant is the runner, manager, and coordinator of a project, the one who actually runs the project.
///
/// The assistant is responsible for all the logistics.
///
/// The assistant communicates among the directors and other agents and tools.
#[async_trait]
pub trait ProjectAssistantTrait: AgentTrait {
    /// Check if the director is ready to begin a project
    ///
    /// This step is called when the assistant receives a message from the director and found the key word that indicates the director understood the prompt.
    /// For example, this method returns true when the assistant receives a message from the director with the key word "READY".
    async fn is_director_ready_to_iterate(&self, director_msg: Message) -> bool;
    async fn is_director_ready_to_conclude(&self, director_msg: Message) -> bool;
    async fn is_final_results_ready(&self, director_msg: Message) -> bool;

    /// Run the project
    ///
    /// The core methods that runs the projects.
    ///
    /// The assistant takes care of each step of the project and communicate among all agents (directors and other agents) and tools.
    ///
    /// The final result of the project is also prepared and returned by the assistant.
    async fn run_project(&self, project: Project) -> Result<ProjectResult, TuoError>;
    async fn run_prep_stage(&self, project: Project) -> Result<Project, TuoError>;

    async fn run_iterating_stage(&self, project: Project) -> Result<Project, TuoError>;
    async fn run_conclusion_stage(&self, project: Project) -> Result<Project, TuoError>;
}

pub struct ProjectAssistant {
    last_message: Message,
    memory: HashMap<String, Memory>,
}


#[async_trait]
impl CompletionModelTrait for ProjectAssistant {
    async fn complete(&self, message: Message) -> Result<Message, TuoError> {
        todo!()
    }

    async fn is_healthy(&self) -> Result<bool, TuoError> {
        todo!()
    }

    async fn get_model_name(&self) -> Result<String, TuoError> {
        todo!()
    }
}

#[async_trait]
impl ProfileTrait for ProjectAssistant {
    async fn get_profile_prompt(&self) -> Result<String, TuoError> {
        todo!()
    }
}

impl AgentTrait for ProjectAssistant {}

#[async_trait]
impl ProjectAssistantTrait for ProjectAssistant {
    async fn is_director_ready_to_iterate(&self, director_msg: Message) -> bool {
        todo!()
    }
    async fn is_director_ready_to_conclude(&self, director_msg: Message) -> bool {
        todo!()
    }
    async fn is_final_results_ready(&self, director_msg: Message) -> bool {
        todo!()
    }


    async fn run_project(&self, mut project: Project) -> Result<ProjectResult, TuoError> {
        // check everything is ready.
        // run the preparation stage
        let prepped_project = self.run_prep_stage(project).await?;
        // run the iterating stage
        let iterated_project = self.run_iterating_stage(prepped_project).await?;
        // run the conclusion stage
        let concluded_project = self.run_conclusion_stage(iterated_project).await?;

        // return the result
        Ok(concluded_project.result.expect("Project result is not found."))
    }

    async fn run_prep_stage(&self, mut project: Project) -> Result<Project, TuoError> {
        let director = project.director.clone();
        let assistant = project.assistant.clone();
        // prepare the director about his role.
        let director_prompt = director.read().await.agent_role_prompt()?;
        let assistant_msg_to_director = assistant.write().await.draft(director_prompt)?;

        // send the message to the director
        let director_initial_message = director.write().await.complete(assistant_msg_to_director).await?;

        // check if the director is ready to iterate
        let is_ready = self.is_director_ready_to_iterate(director_initial_message).await;

        if !is_ready {
            // TODO: this is temporary because we decides on a retry mechanism with possible backoff design.
            Err(TuoError::GenericError("Director is not ready to iterate".to_string()))
        } else {
            // if the director is ready, then we can start the iteration stage.
            project.stage = Stage::Iterating;
            Ok(project)
        }
    }

    async fn run_iterating_stage(&self, project: Project) -> Result<Project, TuoError> {
        let director = project.director.clone();
        let assistant = project.assistant.clone();

        let max_iterations = project.max_iterations;

        for i in 0..max_iterations {
            let message_to_director = assistant.write().await.draft("ITERATE".to_string())?;
            let director_response = director.write().await.complete(message_to_director).await?;
            let is_ready_to_terminate_early = self.is_director_ready_to_conclude(director_response).await;
            if is_ready_to_terminate_early {
                break;
            }
        }

        Ok(project)
    }

    async fn run_conclusion_stage(&self, mut project: Project) -> Result<Project, TuoError> {
        let director = project.director.clone();
        let assistant = project.assistant.clone();

        let max_conclusion_iterations = project.max_conclusion_iterations;

        for i in 0..max_conclusion_iterations {
            let message_to_director = assistant.write().await.draft("CONCLUDE".to_string())?;
            let director_response = director.write().await.complete(message_to_director).await?;
            let is_ready_to_terminate_early = self.is_final_results_ready(director_response).await;
            if is_ready_to_terminate_early {
                break;
            }
        }

        let final_result = director.read().await.read_final_result()?;

        project.result = Some(ProjectResult {
            content: final_result,
        });
        Ok(project)
    }
}