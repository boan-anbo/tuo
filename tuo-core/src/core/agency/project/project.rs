use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::core::agency::agent::AgentTrait;
use crate::core::agency::project::project_assistant::ProjectAssistant;
use crate::core::agency::project::project_director::ProjectDirector;
use crate::core::agency::tool::ToolTrait;
use crate::core::messaging::memory::Memory;
use crate::error::TuoError;

/// The project is the environment in which the agents and tools operate.
///
/// ## Features
/// - An __Agnostic__ environment for the agents towards a collective goal set for the director.
///
/// ## Agnostic about the agents
///
/// The project knows nothing about the agenda of the agents, the format of their prompts, the tools they use, what result is expected etc.
///
/// The agent implementers are responsible for all design.
///
/// The project_assistant knows the logistics. 
///
/// The assistant is responsible for actually executing all the steps and communicate among all agents and tools.
///
/// ## Participants: the assistant is the actual mastermind
/// - Assistant: The special agent that assists the director in **actually** orchestrating the agents and tools to achieve the goal. And communicate with the _outside world_ such as clients representing the project team.
/// - Director: The special agent that sets the goal and **planning** on how to orchestrate the agents and tools to achieve the goal.
/// - Agents: The non-special agents that perform the tasks to achieve the goal.
/// - Tools: The tools that all agents use to perform the tasks to achieve the goal.
///
/// ## Workflow
///
/// ### 1. Project Initialization
///
/// The World is initialized with resources: director, assistant, agents, and tools.
///
/// It's the work of the implementer to make sure director, assistant, agents, and tools are properly initialized and will work together through proper testing.
///
/// ### 2. Project Preparation
///
/// This stage is for the implementer to prepare the world for the agents and tools to operate.
///
/// Usually, it involves feeding the director with an elaborate prompt about what to do.
/// The preparation stage is done when the assistant is sure that the director understood the prompt and is ready to proceed.
/// This is done by calling the `is_director_ready_to_iterate` method of the *assistant*.
///
/// ### 3. Project Iteration
///
/// During running stage, the director designs the steps iteratively to execute.
/// For each step, the assistant execute the step by calling the directed agent or tool to perform the step.
/// Then, the assistant reports back to the director about the result of the step, and the next step. Then director then tell the assistant whether to proceed to the next step or conclude the project.
/// For each step, the method `is_director_ready_to_conclude` of the *assistant* is called to check if the director is ready to conclude the project, if yes, the world enters the _conclusion_ stage.
///
/// #### 3.1. Step Execution
///
/// For any step, it runs like this:
/// 1. The project calls the `step` method of the *assistant*.
/// 2. The assistant decides how to execute the step and calls the `step` method of the directed agent or tool.
///
/// #### 3.2. How assistant execute each step
///
/// ### 4. Project Conclusion
///
/// The conclusion stage is the stage where the assistant and the director conclude the project and calibrate the results according to the initial goal set by the director.
///
/// After a series of concluding steps, the world is concluded and the result is returned. This is done by repeatedly calling the `is_final_results_ready` method of the *assistant*.
///
/// ### 5. Project Result
///
/// The result of the world is returned.
pub struct Project {
    // Resources
    pub director: Arc<RwLock<ProjectDirector>>,
    pub assistant: Arc<RwLock<ProjectAssistant>>,
    pub agents: HashMap<String, Box<dyn AgentTrait>>,
    pub tools: HashMap<String, Box<dyn ToolTrait>>,

    // States
    pub stage: Stage,
    pub stages: Vec<Stage>,
    pub max_iterations: u32,
    pub max_conclusion_iterations: u32,
    // Memory for all messages in the project.
    pub project_memory: Memory,

    // Results
    pub result: Option<ProjectResult>,
}

pub struct ProjectResult {
    pub content: String,
}

pub enum Stage {
    Preparation,
    Iterating,
    Conclusion,
}


#[async_trait]
pub trait ProjectTrait {
    async fn run(&self) -> Result<Project, TuoError> {
        todo!()
    }
}

#[async_trait]
impl ProjectTrait for Project {}


