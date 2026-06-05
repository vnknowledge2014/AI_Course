[[{'duration': 5.04,
   'start': 5.68,
   'text': 'All right, thank you. Thank you everyone for\xa0\n'
           "joining us. Uh, so we're picking up with prompting\xa0\xa0"},
  {'duration': 5.2,
   'start': 10.72,
   'text': 'for agents. Um, hopefully you were here for\xa0\n'
           "prompting 101 or maybe you're just joining us. U,\xa0\xa0"},
  {'duration': 4.88,
   'start': 15.92,
   'text': "but I'll give a little intro. My name is Hannah.\xa0\n"
           "I'm part of the applied AI team in Anthropic. Hi,\xa0\xa0"},
  {'duration': 5.04,
   'start': 20.8,
   'text': "I'm Jeremy. I'm on our applied AI team as well\xa0\n"
           "and I'm a product engineer. Uh, so we're going\xa0\xa0"},
  {'duration': 2.96,
   'start': 25.84,
   'text': "to talk about prompting for agents. So, we're\xa0\n"
           'going to switch gears a little bit, move on from\xa0\xa0'},
  {'duration': 5.6,
   'start': 28.8,
   'text': 'the basics of prompting, um, and talk about how\xa0\n'
           'we do this for agents like playing Pokemon. Uh,\xa0\xa0'},
  {'duration': 4.72,
   'start': 34.4,
   'text': 'so hopefully you were here, uh, for prompting\xa0\n'
           '101 or maybe you have some familiarity with\xa0\xa0'},
  {'duration': 4.88,
   'start': 39.12,
   'text': "basic prompting. So, we're not going to go over\xa0\n"
           'um the really kind of basic console prompting or\xa0\xa0'},
  {'duration': 4.96,
   'start': 44.0,
   'text': 'interacting with Claude and the desktop today.\xa0\n'
           'But just a refresher, uh, we think about prompt\xa0\xa0'},
  {'duration': 4.24,
   'start': 48.96,
   'text': 'engineering as kind of programming in natural\xa0\n'
           "language. you're thinking about what your agent\xa0\xa0"},
  {'duration': 4.48,
   'start': 53.2,
   'text': 'or your model is going to be doing, what kind\xa0\n'
           "of tasks it's accomplishing. You're trying to\xa0\xa0"},
  {'duration': 5.92,
   'start': 57.68,
   'text': 'clearly communicate to the agent, give examples\xa0\n'
           'where necessary, um, and give guidelines. Uh,\xa0\xa0'},
  {'duration': 5.36,
   'start': 63.6,
   'text': 'we do, you know, follow kind of a very specific\xa0\n'
           'structure for console prompting. I want you to\xa0\xa0'},
  {'duration': 3.68,
   'start': 68.96,
   'text': 'remove this from your mind because it could look\xa0\n'
           'very different for an agent. So, for an agent,\xa0\xa0'},
  {'duration': 4.8,
   'start': 72.64,
   'text': 'you may not be laying out this type of very\xa0\n'
           "structured prompt. Uh, it's actually going to\xa0\xa0"},
  {'duration': 3.6,
   'start': 77.44,
   'text': "look a lot different. We're going to allow\xa0\n"
           'a lot of different things to come in. So,\xa0\xa0'},
  {'duration': 3.52,
   'start': 81.04,
   'text': "I'm going to turn it over I'm going to talk about\xa0\n"
           "what agents are and then I'll turn it over to\xa0\xa0"},
  {'duration': 5.12,
   'start': 84.56,
   'text': 'Jeremy to talk about how we do this for agents.\xa0\n'
           'So, hopefully you have a sense in your mind of\xa0\xa0'},
  {'duration': 5.44,
   'start': 89.68,
   'text': 'what an agent is. At Anthropic, we like to say\xa0\n'
           'that agents are models using tools in a loop. So,\xa0\xa0'},
  {'duration': 6.8,
   'start': 95.12,
   'text': 'we give the agent a task and we allow it to work\xa0\n'
           'continuously and use tools as it thinks fit. Um,\xa0\xa0'},
  {'duration': 4.32,
   'start': 101.92,
   'text': 'update its decisions based on the information\xa0\n'
           "that it's getting back from its tool calls and\xa0\xa0"},
  {'duration': 5.68,
   'start': 106.24,
   'text': 'continue working independently until it completes\xa0\n'
           "the task. So that's we kind of keep it as simple\xa0\xa0"},
  {'duration': 4.96,
   'start': 111.92,
   'text': 'as that. Um the environment which is where the\xa0\n'
           'agent is working, the tools that the agent has\xa0\xa0'},
  {'duration': 4.4,
   'start': 116.88,
   'text': 'and the system prompt is just where we tell the\xa0\n'
           'agent what it should be doing or what it should be\xa0\xa0'},
  {'duration': 5.12,
   'start': 121.28,
   'text': 'accomplishing. And we typically find the simpler\xa0\n'
           'you can keep this the better. Allow the agent to\xa0\xa0'},
  {'duration': 6.32,
   'start': 126.4,
   'text': 'do its work. Allow the model to be the model and\xa0\n'
           'kind of work through this task. So when do you use\xa0\xa0'},
  {'duration': 5.28,
   'start': 132.72,
   'text': 'agents? You do not always need to use an agent.\xa0\n'
           "In fact, there's many scenarios in which you won't\xa0\xa0"},
  {'duration': 4.56,
   'start': 138.0,
   'text': 'actually want to use an agent. There are other\xa0\n'
           'approaches that would be more appropriate. Um,\xa0\xa0'},
  {'duration': 4.72,
   'start': 142.56,
   'text': 'agents are really best for complex and\xa0\n'
           "valuable tasks. It's not something you\xa0\xa0"},
  {'duration': 4.56,
   'start': 147.28,
   'text': 'should deploy in every possible scenario. You\xa0\n'
           'will not get the results that you want. Um,\xa0\xa0'},
  {'duration': 3.84,
   'start': 151.84,
   'text': "and you'll spend a lot more resources than\xa0\n"
           "you maybe need to. So, we'll talk a little\xa0\xa0"},
  {'duration': 5.2,
   'start': 155.68,
   'text': 'bit about checklist or or kind of ways of thinking\xa0\n'
           'about when you should be using an agent and maybe\xa0\xa0'},
  {'duration': 5.68,
   'start': 160.88,
   'text': "you don't want to be using an agent. So, is the\xa0\n"
           'task complex? Is this a task that you, a human,\xa0\xa0'},
  {'duration': 4.64,
   'start': 166.56,
   'text': 'can think through a step-by-step process to\xa0\n'
           "complete? If so, you probably don't need an\xa0\xa0"},
  {'duration': 4.64,
   'start': 171.2,
   'text': "agent. You want to use an agent where it's not\xa0\n"
           "clear to you how you'll go about accomplishing the\xa0\xa0"},
  {'duration': 4.48,
   'start': 175.84,
   'text': 'task. You might know where you want to go, but you\xa0\n'
           "don't know exactly how you're going to get there,\xa0\xa0"},
  {'duration': 4.8,
   'start': 180.32,
   'text': 'what tools, and what information you might need\xa0\n'
           'to arrive at the end state. Is a task valuable?\xa0\xa0'},
  {'duration': 4.64,
   'start': 185.12,
   'text': 'Are you going to get a lot of value out of the\xa0\n'
           'agent accomplishing this task? Or is this a kind\xa0\xa0'},
  {'duration': 5.2,
   'start': 189.76,
   'text': 'of a low value uh task or workflow? In that case,\xa0\n'
           "a workflow might also be better. You don't really\xa0\xa0"},
  {'duration': 4.64,
   'start': 194.96,
   'text': 'want to be using the resources of an agent unless\xa0\n'
           "this is something you get that's highly leveraged.\xa0\xa0"},
  {'duration': 3.76,
   'start': 199.6,
   'text': "It's maybe revenue generating. It's something\xa0\n"
           "that's really valuable to your user. Again,\xa0\xa0"},
  {'duration': 5.76,
   'start': 203.36,
   'text': "it's something that's complex. Uh the last next\xa0\n"
           'piece is are the parts of the task doable? So,\xa0\xa0'},
  {'duration': 5.04,
   'start': 209.12,
   'text': 'when you think about the task that has to occur,\xa0\n'
           'would you be able to give the agents the tools\xa0\xa0'},
  {'duration': 5.28,
   'start': 214.16,
   'text': 'that it needs in order to accomplish this task?\xa0\n'
           "If you can't define the tools or if you can't\xa0\xa0"},
  {'duration': 4.4,
   'start': 219.44,
   'text': 'give the agent access to the information or\xa0\n'
           'the tool that it would need, you may want to\xa0\xa0'},
  {'duration': 5.36,
   'start': 223.84,
   'text': 'scope the task down. Um, if you can define and\xa0\n'
           'give to the agent the tools that it would want,\xa0\xa0'},
  {'duration': 5.04,
   'start': 229.2,
   'text': "that's a better use case for an agent. The last\xa0\n"
           'thing you might want to think about is the cost of\xa0\xa0'},
  {'duration': 6.24,
   'start': 234.24,
   'text': 'errors or how easy it is to discover errors. So,\xa0\n'
           "if it's really uh difficult to correct an error or\xa0\xa0"},
  {'duration': 4.48,
   'start': 240.48,
   'text': 'detect an error, that is maybe not a place where\xa0\n'
           'you want the agent to be working independently.\xa0\xa0'},
  {'duration': 4.08,
   'start': 244.96,
   'text': 'you might want to have a human in the loop in\xa0\n'
           'that case. If it the error is something that\xa0\xa0'},
  {'duration': 5.28,
   'start': 249.04,
   'text': "you can recover from or if it's not too costly to\xa0\n"
           'have an error occurring, then you might continue\xa0\xa0'},
  {'duration': 5.76,
   'start': 254.32,
   'text': 'to allow the agent to work independently. So to\xa0\n'
           "make this a little bit more real, uh we'll talk\xa0\xa0"},
  {'duration': 4.8,
   'start': 260.08,
   'text': "about a few examples. I'm not going to go through\xa0\n"
           "each single one of these, but let's pick out a few\xa0\xa0"},
  {'duration': 5.52,
   'start': 264.88,
   'text': 'that will be pretty clear or intuitive for most of\xa0\n'
           'us. So coding, obviously, um all of you are very\xa0\xa0'},
  {'duration': 5.04,
   'start': 270.4,
   'text': 'familiar with using agents and coding. Uh coding\xa0\n'
           'is a great use case. We can think about something\xa0\xa0'},
  {'duration': 5.84,
   'start': 275.44,
   'text': 'uh like a design document. And although you know\xa0\n'
           'where you want to get to, which is raising a PR,\xa0\xa0'},
  {'duration': 3.52,
   'start': 281.28,
   'text': "you don't know exactly how you're going to get\xa0\n"
           "there. It's not clear to you what you'll build\xa0\xa0"},
  {'duration': 4.4,
   'start': 284.8,
   'text': "first, how you'll iterate on that, what changes\xa0\n"
           'you might make along the way depending on what\xa0\xa0'},
  {'duration': 7.52,
   'start': 289.2,
   'text': "you find. Um this is high value. You're all very\xa0\n"
           'skilled. If an agent, okay, if an agent is able,\xa0\xa0'},
  {'duration': 8.08,
   'start': 296.72,
   'text': 'this is like more like what the midway is like\xa0\n'
           'at night. I feel I feel more at home now. Um,\xa0\xa0'},
  {'duration': 4.72,
   'start': 304.8,
   'text': 'uh, Claude Claude is great at coding. Um, and this\xa0\n'
           'is a high value use case, right? If your agent is\xa0\xa0'},
  {'duration': 5.2,
   'start': 309.52,
   'text': 'actually able to go from a design document to\xa0\n'
           "a PR, that's a lot of time that you, a highly\xa0\xa0"},
  {'duration': 4.32,
   'start': 314.72,
   'text': "skilled engineer, are saved and you're able to\xa0\n"
           "then spend your time on something else that's\xa0\xa0"},
  {'duration': 6.16,
   'start': 319.04,
   'text': 'higher leverage. So, great use case for agents.\xa0\n'
           "A couple other examples I'll mention here. Um,\xa0\xa0"},
  {'duration': 5.44,
   'start': 325.2,
   'text': "maybe we'll talk about the the cost of error.\xa0\n"
           'So, search, if we make an error in the search,\xa0\xa0'},
  {'duration': 4.96,
   'start': 330.64,
   'text': "there's ways that we can correct that, right? So\xa0\n"
           'we can use citations, we can use other methods of\xa0\xa0'},
  {'duration': 4.32,
   'start': 335.6,
   'text': 'double-checking the results. So if the agent makes\xa0\n'
           'a mistake in the search process, this is something\xa0\xa0'},
  {'duration': 5.6,
   'start': 339.92,
   'text': "we can recover from and it's probably not too\xa0\n"
           'costly. Computer use, um, this is also a place\xa0\xa0'},
  {'duration': 4.96,
   'start': 345.52,
   'text': 'where we can recover from errors. We might just go\xa0\n'
           "back, we might try clicking again. It's not, uh,\xa0\xa0"},
  {'duration': 5.76,
   'start': 350.48,
   'text': 'too difficult to allow Claude just to click a few\xa0\n'
           "times until it's able to use the tool properly.\xa0\xa0"},
  {'duration': 5.2,
   'start': 356.24,
   'text': 'Um, data analysis, I think, is another interesting\xa0\n'
           'example, kind of analogous to coding. We might\xa0\xa0'},
  {'duration': 4.48,
   'start': 361.44,
   'text': 'know uh the end result that we want to get to.\xa0\n'
           'We know a set of insights that we want to gather\xa0\xa0'},
  {'duration': 4.4,
   'start': 365.92,
   'text': 'out of data or a visualization that we want to\xa0\n'
           "produce from data. We don't know exactly what the\xa0\xa0"},
  {'duration': 4.24,
   'start': 370.32,
   'text': 'data might look like. Uh so the data could have\xa0\n'
           'different formats. It could have errors in it.\xa0\xa0'},
  {'duration': 5.44,
   'start': 374.56,
   'text': 'It could have other uh it could have granularity\xa0\n'
           "issues that we're not sure how to disagregate. We\xa0\xa0"},
  {'duration': 3.76,
   'start': 380.0,
   'text': "don't know the exact process that we're going to\xa0\n"
           'take in analyzing that data, but we know where we\xa0\xa0'},
  {'duration': 6.24,
   'start': 383.76,
   'text': 'want to get in the end. Um so this is another\xa0\n'
           'example of a great use case for agents. Uh,\xa0\xa0'},
  {'duration': 4.16,
   'start': 390.0,
   'text': "so hopefully these make sense to you and I'm going\xa0\n"
           'to turn it over to Jeremy now. He has some really\xa0\xa0'},
  {'duration': 5.76,
   'start': 394.16,
   'text': "rich experience building agents and he's going to\xa0\n"
           'share some best practices for actually prompting\xa0\xa0'},
  {'duration': 6.16,
   'start': 399.92,
   'text': 'them well and how to structure a great prompt\xa0\n'
           'for an agent. Thanks Hannah. Hi all. Um, yeah,\xa0\xa0'},
  {'duration': 5.36,
   'start': 406.08,
   'text': 'so prompting for agents. Um, I think some things\xa0\n'
           "that we think about here, I I'll go over a few of\xa0\xa0"},
  {'duration': 5.04,
   'start': 411.44,
   'text': "them. We've learned these experiences mostly from\xa0\n"
           'building agents ourselves. So some agents that you\xa0\xa0'},
  {'duration': 5.2,
   'start': 416.48,
   'text': 'can try from enthropic are cla code which works in\xa0\n'
           'your terminal and sort of agentically browses your\xa0\xa0'},
  {'duration': 5.84,
   'start': 421.68,
   'text': 'files and uses the bash tool to really accomplish\xa0\n'
           'tasks um in coding. Similarly we have our new\xa0\xa0'},
  {'duration': 4.96,
   'start': 427.52,
   'text': 'advanced research feature in cloud.ai and this\xa0\n'
           'allows you to do hours of research. For example,\xa0\xa0'},
  {'duration': 5.52,
   'start': 432.48,
   'text': 'you can find hundreds of startups building agents\xa0\n'
           'or you can find hundreds of potential prospects\xa0\xa0'},
  {'duration': 6.16,
   'start': 438.0,
   'text': 'for your company. And this allows the model to\xa0\n'
           'do research across your tools, your Google Drive,\xa0\xa0'},
  {'duration': 4.24,
   'start': 444.16,
   'text': 'web search and stuff like that. And so in the\xa0\n'
           'process of building these products, one things\xa0\xa0'},
  {'duration': 4.4,
   'start': 448.4,
   'text': 'that we learned is that you need to think like\xa0\n'
           'your agents. This is maybe the most important\xa0\xa0'},
  {'duration': 4.56,
   'start': 452.8,
   'text': 'principle. Um the idea is that essentially you\xa0\n'
           'need to understand and develop a mental model\xa0\xa0'},
  {'duration': 4.96,
   'start': 457.36,
   'text': "of what your agent is doing and what it's like to\xa0\n"
           'be in that environment. So the environment for the\xa0\xa0'},
  {'duration': 4.8,
   'start': 462.32,
   'text': 'agent is a set of tools and the responses it gets\xa0\n'
           'back from those tools. In the context of cloud\xa0\xa0'},
  {'duration': 5.52,
   'start': 467.12,
   'text': 'code, the way you might do this is by actually\xa0\n'
           'simulating the process and just imagining if you\xa0\xa0'},
  {'duration': 5.52,
   'start': 472.64,
   'text': "were in cloud code's shoes given the exact tool\xa0\n"
           'descriptions it has and the tool schemas it has,\xa0\xa0'},
  {'duration': 4.56,
   'start': 478.16,
   'text': 'would you be confused or would you be able to\xa0\n'
           "do do the task that it's doing? If a human can't\xa0\xa0"},
  {'duration': 4.8,
   'start': 482.72,
   'text': 'understand what your agent should be doing, then\xa0\n'
           'an AI will not be able to either. And so this is\xa0\xa0'},
  {'duration': 4.32,
   'start': 487.52,
   'text': 'really important for thinking about tool design,\xa0\n'
           'thinking about prompting is to simulate and go\xa0\xa0'},
  {'duration': 5.52,
   'start': 491.84,
   'text': 'through their environment. Another is that you\xa0\n'
           'need to give your agents reasonable heristics.\xa0\xa0'},
  {'duration': 4.24,
   'start': 497.36,
   'text': 'And so, you know, Hannah mentioned that prompt\xa0\n'
           'engineering is conceptual engineering. What does\xa0\xa0'},
  {'duration': 4.4,
   'start': 501.6,
   'text': "that really mean? It's one of the reasons why\xa0\n"
           'prompt engineering is not going away and why I\xa0\xa0'},
  {'duration': 4.72,
   'start': 506.0,
   'text': 'personally expect prompting to get more important,\xa0\n'
           'not less important as models get smarter. This is\xa0\xa0'},
  {'duration': 4.32,
   'start': 510.72,
   'text': "because prompting is not just about text. It's not\xa0\n"
           "just about the words that you give the model. It's\xa0\xa0"},
  {'duration': 5.36,
   'start': 515.04,
   'text': 'about deciding what concepts the model should have\xa0\n'
           'and what behaviors it should follow to perform\xa0\xa0'},
  {'duration': 5.84,
   'start': 520.4,
   'text': 'well in a specific environment. So for example,\xa0\n'
           'cloud code has the concept of irreversibility.\xa0\xa0'},
  {'duration': 5.04,
   'start': 526.24,
   'text': 'It should not take irreversible actions that\xa0\n'
           'might harm the user or harm their environment.\xa0\xa0'},
  {'duration': 4.88,
   'start': 531.28,
   'text': 'So it will avoid these kinds of harmful actions\xa0\n'
           'or anything that might cause irreversible damage\xa0\xa0'},
  {'duration': 4.56,
   'start': 536.16,
   'text': 'to your environment or to your code or anything\xa0\n'
           'like that. So that concept of irreversibility is\xa0\xa0'},
  {'duration': 4.08,
   'start': 540.72,
   'text': 'something that you need to instill in the model\xa0\n'
           'and be very clear about and think about the edge\xa0\xa0'},
  {'duration': 4.56,
   'start': 544.8,
   'text': 'cases. How might the model in misinterpret\xa0\n'
           'this concept? How might it not know what it\xa0\xa0'},
  {'duration': 5.04,
   'start': 549.36,
   'text': 'means? For example, if you want the model to be\xa0\n'
           'very eager and you want it to be very agentic,\xa0\xa0'},
  {'duration': 4.16,
   'start': 554.4,
   'text': 'well, it might go over the top a little bit. It\xa0\n'
           "might misinterpret what you're saying and do more\xa0\xa0"},
  {'duration': 3.68,
   'start': 558.56,
   'text': 'than what you expect. And so, you have to be very\xa0\n'
           "crisp and clear about the concepts you're giving\xa0\xa0"},
  {'duration': 5.36,
   'start': 562.24,
   'text': 'the models. Um, some examples of these reasonable\xa0\n'
           "heristics that we've learned. One is that while\xa0\xa0"},
  {'duration': 4.08,
   'start': 567.6,
   'text': 'we were building research, we noticed that the\xa0\n'
           'model would often do a ton of web searches when\xa0\xa0'},
  {'duration': 5.04,
   'start': 571.68,
   'text': 'it was unnecessary. For example, it would find the\xa0\n'
           'actual answer it needed. like maybe you would find\xa0\xa0'},
  {'duration': 5.36,
   'start': 576.72,
   'text': 'a list of scaleups in the United States and then\xa0\n'
           'it would keep going even though it already had the\xa0\xa0'},
  {'duration': 5.12,
   'start': 582.08,
   'text': "answer and that's because we hadn't told the model\xa0\n"
           'explicitly when you find the answer you can stop\xa0\xa0'},
  {'duration': 4.72,
   'start': 587.2,
   'text': 'you no longer need to keep searching uh similarly\xa0\n'
           'we had to give the model sort of budgets to think\xa0\xa0'},
  {'duration': 5.28,
   'start': 591.92,
   'text': 'about for example we told it that for simple\xa0\n'
           'queries it should use under five tool calls\xa0\xa0'},
  {'duration': 5.44,
   'start': 597.2,
   'text': 'but for more complex queries it might use up to 10\xa0\n'
           'or 15 so these kinds of heruristics that you might\xa0\xa0'},
  {'duration': 4.88,
   'start': 602.64,
   'text': 'assume the model already understands you really\xa0\n'
           'have to articulate clearly. A good way to think\xa0\xa0'},
  {'duration': 4.72,
   'start': 607.52,
   'text': "about this is that if you're managing maybe a new\xa0\n"
           "intern who's fresh out of college and has not had\xa0\xa0"},
  {'duration': 5.36,
   'start': 612.24,
   'text': 'a job before, how would you articulate to them\xa0\n'
           'how to get around all the problems they might get\xa0\xa0'},
  {'duration': 4.16,
   'start': 617.6,
   'text': 'run into in their first job? And how would you\xa0\n'
           'be very crisp and clear with them about how to\xa0\xa0'},
  {'duration': 3.68,
   'start': 621.76,
   'text': "accomplish that? That's often how you should\xa0\n"
           'think about giving heristics to your agents,\xa0\xa0'},
  {'duration': 3.68,
   'start': 625.44,
   'text': 'which are just general principles that it\xa0\n'
           'should follow. They may not be strict rules,\xa0\xa0'},
  {'duration': 5.36,
   'start': 629.12,
   'text': "but they're, you know, sort of practices.\xa0\n"
           'Another point is that tool selection is key.\xa0\xa0'},
  {'duration': 5.76,
   'start': 634.48,
   'text': 'So as models get more powerful able to handle more\xa0\n'
           'and more tools. Sonnet 4 and Opus 4 can handle\xa0\xa0'},
  {'duration': 4.56,
   'start': 640.24,
   'text': 'you know up to a hundred tools even more than\xa0\n'
           'that if you have great prompting. But in order\xa0\xa0'},
  {'duration': 3.84,
   'start': 644.8,
   'text': 'to use these tools you have to be clear about\xa0\n'
           'which tools it should use for different tasks.\xa0\xa0'},
  {'duration': 4.24,
   'start': 648.64,
   'text': 'So for example for research we can give the model\xa0\n'
           'access to Google Drive. We can give it access to\xa0\xa0'},
  {'duration': 6.72,
   'start': 652.88,
   'text': 'MCP tools like Sentry or Data Dog or GitHub. It\xa0\n'
           'can search across all these tools, but the model\xa0\xa0'},
  {'duration': 4.88,
   'start': 659.6,
   'text': "doesn't know already which tools are important\xa0\n"
           'for which tasks. Especially in your specific\xa0\xa0'},
  {'duration': 5.2,
   'start': 664.48,
   'text': 'company context. For example, if your company uses\xa0\n'
           'Slack a lot, maybe it should default to searching\xa0\xa0'},
  {'duration': 6.08,
   'start': 669.68,
   'text': 'Slack for company related information. All these\xa0\n'
           'questions about how the model should use tools,\xa0\xa0'},
  {'duration': 5.36,
   'start': 675.76,
   'text': 'you have to give it explicit principles about\xa0\n'
           'when to use which tools and in which contexts. Um,\xa0\xa0'},
  {'duration': 4.0,
   'start': 681.12,
   'text': "and this is really important and it's often\xa0\n"
           "something I see where people don't prompt the\xa0\xa0"},
  {'duration': 4.72,
   'start': 685.12,
   'text': 'agent at all about which tool to use and they\xa0\n'
           'just give the model some tools with some very\xa0\xa0'},
  {'duration': 4.4,
   'start': 689.84,
   'text': 'short descriptions and then they wonder like\xa0\n'
           "why isn't the model using the right tool? Well,\xa0\xa0"},
  {'duration': 4.4,
   'start': 694.24,
   'text': "it's likely because the model doesn't know what\xa0\n"
           'it should be doing in that context. Another point\xa0\xa0'},
  {'duration': 4.96,
   'start': 698.64,
   'text': 'here is that you can guide the thinking process.\xa0\n'
           'So people often sort of turn extended thinking on\xa0\xa0'},
  {'duration': 4.56,
   'start': 703.6,
   'text': 'and then let their agents run and assume it will\xa0\n'
           'get out of the box better performance. Actually\xa0\xa0'},
  {'duration': 4.32,
   'start': 708.16,
   'text': 'that assumption is true. Most of the time you will\xa0\n'
           'get out of the box better performance, but you can\xa0\xa0'},
  {'duration': 4.8,
   'start': 712.48,
   'text': 'squeeze even more performance out of it if you\xa0\n'
           'just prompt the agent to use its thinking well.\xa0\xa0'},
  {'duration': 5.2,
   'start': 717.28,
   'text': 'So for example, for search, what we do is tell\xa0\n'
           'the model to plan out its search process. So in\xa0\xa0'},
  {'duration': 5.52,
   'start': 722.48,
   'text': 'advance, it should decide how complicated is this\xa0\n'
           'query? How many tool calls should I use here? What\xa0\xa0'},
  {'duration': 4.88,
   'start': 728.0,
   'text': 'sources should I look for? How will I know when\xa0\n'
           "I'm successful? We tell it to plan out all these\xa0\xa0"},
  {'duration': 5.52,
   'start': 732.88,
   'text': 'exact things in its first thinking block. And then\xa0\n'
           'a new capability that the cloud 4 models have is\xa0\xa0'},
  {'duration': 5.36,
   'start': 738.4,
   'text': 'the ability to use interled thinking between tool\xa0\n'
           'calls. So after getting results from the web, we\xa0\xa0'},
  {'duration': 4.8,
   'start': 743.76,
   'text': 'often find that models assume that all web search\xa0\n'
           "results are true, right? They don't have any,\xa0\xa0"},
  {'duration': 4.08,
   'start': 748.56,
   'text': "you know, we we haven't told them explicitly that\xa0\n"
           "this isn't the case. And so they might take these\xa0\xa0"},
  {'duration': 4.24,
   'start': 752.64,
   'text': 'web results and run with them immediately. So,\xa0\n'
           'one thing we prompted our models to do is to use\xa0\xa0'},
  {'duration': 5.04,
   'start': 756.88,
   'text': 'this interleaf thinking to really reflect on the\xa0\n'
           'quality of the search results and decide if they\xa0\xa0'},
  {'duration': 3.76,
   'start': 761.92,
   'text': 'need to verify them, if they need to get more\xa0\n'
           'information, or if they should add a disclaimer\xa0\xa0'},
  {'duration': 5.04,
   'start': 765.68,
   'text': 'about how the results might not be accurate. Um,\xa0\n'
           'another point with when prompting agents is that\xa0\xa0'},
  {'duration': 6.24,
   'start': 770.72,
   'text': 'agents are more unpredictable than workflows\xa0\n'
           'or just, you know, classification type prompts.\xa0\xa0'},
  {'duration': 5.04,
   'start': 776.96,
   'text': 'Most changes will have unintended side effects.\xa0\n'
           'This is because agents will operate in a loop\xa0\xa0'},
  {'duration': 5.84,
   'start': 782.0,
   'text': 'autonomously. And so for example, if you tell the\xa0\n'
           'agent, you know, keep searching until you find the\xa0\xa0'},
  {'duration': 4.56,
   'start': 787.84,
   'text': 'correct answer, you know, find the highest quality\xa0\n'
           'possible source and always keep searching until\xa0\xa0'},
  {'duration': 5.44,
   'start': 792.4,
   'text': 'you find that source. What you might run into is\xa0\n'
           'the unintended side effect of the agent just not\xa0\xa0'},
  {'duration': 4.8,
   'start': 797.84,
   'text': 'finding any sources. Maybe this perfect source\xa0\n'
           "doesn't exist for the for the query. And so it\xa0\xa0"},
  {'duration': 4.16,
   'start': 802.64,
   'text': 'will just keep searching until it hits its context\xa0\n'
           "window. And that's actually what we ran into as\xa0\xa0"},
  {'duration': 4.0,
   'start': 806.8,
   'text': 'well. And so you have to tell the agent if you\xa0\n'
           "don't find the perfect source, that's okay. You\xa0\xa0"},
  {'duration': 5.36,
   'start': 810.8,
   'text': 'can stop after a few tool calls. Um, so just be\xa0\n'
           'aware that your prompts may have unintended side\xa0\xa0'},
  {'duration': 5.12,
   'start': 816.16,
   'text': 'effects and you may have to roll those back.\xa0\n'
           'Another point is to help the agent manage its\xa0\xa0'},
  {'duration': 5.6,
   'start': 821.28,
   'text': 'context window. The Cloud 4 models have a 200k\xa0\n'
           'token context window. Um, this is long enough for\xa0\xa0'},
  {'duration': 5.04,
   'start': 826.88,
   'text': "a lot of longrunning tasks, but when you're using\xa0\n"
           'an agent to do work autonomously, you may hit this\xa0\xa0'},
  {'duration': 4.16,
   'start': 831.92,
   'text': 'context window and there are several strategies\xa0\n'
           'you can use to sort of extend the effective\xa0\xa0'},
  {'duration': 4.88,
   'start': 836.08,
   'text': 'context window. One of them that we use for cloud\xa0\n'
           'code is called compaction. And this is just a tool\xa0\xa0'},
  {'duration': 7.04,
   'start': 840.96,
   'text': 'that the model has um that will automatically be\xa0\n'
           'called once it hits around 190,000 tokens. So near\xa0\xa0'},
  {'duration': 4.16,
   'start': 848.0,
   'text': 'the context window. And this will summarize\xa0\n'
           'or compress everything in the context window\xa0\xa0'},
  {'duration': 5.12,
   'start': 852.16,
   'text': 'to a really dense but accurate summary that is\xa0\n'
           'then passed to a new instance of claude with the\xa0\xa0'},
  {'duration': 4.88,
   'start': 857.28,
   'text': 'summary. And it continues the process. And we find\xa0\n'
           'that this essentially allows you to run infinitely\xa0\xa0'},
  {'duration': 4.88,
   'start': 862.16,
   'text': 'with cloud code. You almost never run out of\xa0\n'
           'context. um occasionally it will miss details\xa0\xa0'},
  {'duration': 4.64,
   'start': 867.04,
   'text': 'from the previous session but the vast majority of\xa0\n'
           'the time this will keep all the important details\xa0\xa0'},
  {'duration': 4.4,
   'start': 871.68,
   'text': 'and the model will sort of remember what happened\xa0\n'
           'in the last session. Similarly you can sort of\xa0\xa0'},
  {'duration': 5.84,
   'start': 876.08,
   'text': 'write to an external file. So the model can have\xa0\n'
           'access to an extra file and these cloud for models\xa0\xa0'},
  {'duration': 5.12,
   'start': 881.92,
   'text': 'are especially good at writing memory to a file\xa0\n'
           'and they can use this file to essentially extend\xa0\xa0'},
  {'duration': 5.68,
   'start': 887.04,
   'text': 'their context window. Another point is that you\xa0\n'
           "can use sub aents. Um, we won't talk about this\xa0\xa0"},
  {'duration': 5.12,
   'start': 892.72,
   'text': 'a lot here, but essentially if you have agents\xa0\n'
           'that are always hitting their context windows, you\xa0\xa0'},
  {'duration': 5.92,
   'start': 897.84,
   'text': 'may delegate some of what the agent is doing to\xa0\n'
           'another agent. Um, which can sort of, for example,\xa0\xa0'},
  {'duration': 4.96,
   'start': 903.76,
   'text': 'you can have one agent be the lead agent and then\xa0\n'
           'sub agents do the actual searching process. Then\xa0\xa0'},
  {'duration': 4.16,
   'start': 908.72,
   'text': 'the sub agents can compress the results to the\xa0\n'
           "lead agent in a really dense form that doesn't\xa0\xa0"},
  {'duration': 5.44,
   'start': 912.88,
   'text': 'use as many tokens and the lead agent can give the\xa0\n'
           'final report to the user. So we actually use this\xa0\xa0'},
  {'duration': 5.36,
   'start': 918.32,
   'text': 'process in our research system and this allows you\xa0\n'
           "to sort of compress what's going on in the search\xa0\xa0"},
  {'duration': 5.12,
   'start': 923.68,
   'text': 'and then only use the context window for the lead\xa0\n'
           'agent for actually writing the report. So this\xa0\xa0'},
  {'duration': 4.88,
   'start': 928.8,
   'text': 'kind of multi- aent system can be effective\xa0\n'
           'for limiting the context window. Finally,\xa0\xa0'},
  {'duration': 3.92,
   'start': 933.68,
   'text': 'you can let Claude be Claude. And essentially\xa0\n'
           'what this means is that Claude is great at being\xa0\xa0'},
  {'duration': 4.64,
   'start': 937.6,
   'text': "an agent already. You don't have to do a ton of\xa0\n"
           'work at the very beginning. So, I would recommend\xa0\xa0'},
  {'duration': 4.64,
   'start': 942.24,
   'text': 'just trying out your system with sort of a bare\xa0\n'
           'bones prompt and barebones tools and seeing where\xa0\xa0'},
  {'duration': 4.32,
   'start': 946.88,
   'text': "it goes wrong and then working from there. Don't\xa0\n"
           "sort of assume that Claude can't do it ahead of\xa0\xa0"},
  {'duration': 6.96,
   'start': 951.2,
   'text': 'time because cloud often will surprise you with\xa0\n'
           'how good it is. Um, I talked already about tool\xa0\xa0'},
  {'duration': 4.24,
   'start': 958.16,
   'text': 'design, but essentially the key point here is you\xa0\n'
           'want to make sure that your tools are good. Um,\xa0\xa0'},
  {'duration': 4.48,
   'start': 962.4,
   'text': 'what is a good tool? It will have a simple\xa0\n'
           'accurate tool name that reflects what it does.\xa0\xa0'},
  {'duration': 4.48,
   'start': 966.88,
   'text': "You'll have tested it and make sure that it works\xa0\n"
           "well. um it'll have a well-formed description\xa0\xa0"},
  {'duration': 5.12,
   'start': 971.36,
   'text': 'so that a human reading this tool like imagine\xa0\n'
           'you give a function to another engineer on your\xa0\xa0'},
  {'duration': 4.96,
   'start': 976.48,
   'text': 'team would they understand this function and be\xa0\n'
           'able to use it. You should ask the same question\xa0\xa0'},
  {'duration': 5.6,
   'start': 981.44,
   'text': 'about the agent computer interfaces or the tools\xa0\n'
           'that you are giving your agent. Make sure that\xa0\xa0'},
  {'duration': 5.68,
   'start': 987.04,
   'text': "they're usable and clear. Um we also often find\xa0\n"
           'that people will give an agent a bunch of tools\xa0\xa0'},
  {'duration': 5.36,
   'start': 992.72,
   'text': 'that have very similar names or descriptions.\xa0\n'
           'So for example, you give it six search tools\xa0\xa0'},
  {'duration': 4.72,
   'start': 998.08,
   'text': 'and each of the search tools searches a slightly\xa0\n'
           'different database. This will confuse the model.\xa0\xa0'},
  {'duration': 6.16,
   'start': 1002.8,
   'text': 'So try to keep your tools fairly distinct\xa0\n'
           'um and combine similar tools into just one.\xa0\xa0'},
  {'duration': 4.08,
   'start': 1009.84,
   'text': 'So, one quick example here is just that you can\xa0\n'
           'have an agent, for example, use these different\xa0\xa0'},
  {'duration': 6.16,
   'start': 1013.92,
   'text': 'tools to first search the inventory in a database,\xa0\n'
           'run a query. Based on the information it finds, it\xa0\xa0'},
  {'duration': 5.36,
   'start': 1020.08,
   'text': 'can reflect on the inventory, think about it for\xa0\n'
           'a little bit, then decide to generate an invoice,\xa0\xa0'},
  {'duration': 4.88,
   'start': 1025.44,
   'text': 'generate this invoice, think about what it should\xa0\n'
           'do next, and then decide to send an email. And so,\xa0\xa0'},
  {'duration': 4.0,
   'start': 1030.32,
   'text': 'this loop involves the agent getting information\xa0\n'
           'from the database, which is its external\xa0\xa0'},
  {'duration': 5.2,
   'start': 1034.32,
   'text': 'environment, using its tools, and then updating\xa0\n'
           'based on that information. until it accomplishes\xa0\xa0'},
  {'duration': 4.96,
   'start': 1039.52,
   'text': "the task. And that's sort of how agents work\xa0\n"
           "in general. So, let's walk through a demo real\xa0\xa0"},
  {'duration': 6.08,
   'start': 1044.48,
   'text': "quick. I'll switch to my computer. Um, so you can\xa0\n"
           'see here that this is our console. The console is\xa0\xa0'},
  {'duration': 4.56,
   'start': 1050.56,
   'text': 'a great tool for sort of simulating your prompts\xa0\n'
           'and seeing what they would look like in a UI. Um,\xa0\xa0'},
  {'duration': 4.96,
   'start': 1055.12,
   'text': 'and I use this while we were iterating on research\xa0\n'
           "to sort of understand what's really going on and\xa0\xa0"},
  {'duration': 3.92,
   'start': 1060.08,
   'text': 'what the agents doing. This is a great way to\xa0\n'
           'think like your agents and sort of put yourself\xa0\xa0'},
  {'duration': 4.96,
   'start': 1064.0,
   'text': 'in their shoes. So, you can see we have a big\xa0\n'
           "prompt here. Um, it's not sort of super long.\xa0\xa0"},
  {'duration': 4.56,
   'start': 1068.96,
   'text': "It's around a thousand tokens. It involves the\xa0\n"
           'researcher going through a research process. We\xa0\xa0'},
  {'duration': 4.24,
   'start': 1073.52,
   'text': 'tell it exactly what should what it what it should\xa0\n'
           'plan ahead of time. We tell it how many tool\xa0\xa0'},
  {'duration': 4.72,
   'start': 1077.76,
   'text': 'calls it should typically use. We give it some\xa0\n'
           'guidelines about what facts it should think about,\xa0\xa0'},
  {'duration': 4.24,
   'start': 1082.48,
   'text': 'what makes a high quality source, stuff like\xa0\n'
           'that. And then we tell it to use parallel tool\xa0\xa0'},
  {'duration': 5.36,
   'start': 1086.72,
   'text': 'calls. So, you know, run multiple web searches in\xa0\n'
           'parallel at the same time rather than running them\xa0\xa0'},
  {'duration': 5.84,
   'start': 1092.08,
   'text': 'all sequentially. Then we give it this question.\xa0\n'
           'How many bananas can fit in a Rivian R1S? This\xa0\xa0'},
  {'duration': 3.84,
   'start': 1097.92,
   'text': 'is not a question that the model will be able\xa0\n'
           'to answer because the Rivian R1S came out very\xa0\xa0'},
  {'duration': 5.04,
   'start': 1101.76,
   'text': "recently. It's a car. It doesn't know in advance\xa0\n"
           "all the specifications and everything. So, it'll\xa0\xa0"},
  {'duration': 4.4,
   'start': 1106.8,
   'text': "have to search the web. Let's run it and see what\xa0\n"
           "happens. You'll see that at the very beginning,\xa0\xa0"},
  {'duration': 4.32,
   'start': 1111.2,
   'text': 'it will think and break down this request. And\xa0\n'
           'so, it realizes, okay, web search is going to\xa0\xa0'},
  {'duration': 8.08,
   'start': 1115.52,
   'text': 'be helpful here. I should get cargo capacity.\xa0\n'
           'I should search. Um, woo. Um, and you see here\xa0\xa0'},
  {'duration': 4.32,
   'start': 1123.6,
   'text': 'it ran two web searches in parallel at the same\xa0\n'
           'time. That allowed it to get these results back\xa0\xa0'},
  {'duration': 4.56,
   'start': 1127.92,
   'text': "very quickly. And then it's reflecting on the\xa0\n"
           "results. So it's realizing, okay, I found the\xa0\xa0"},
  {'duration': 6.08,
   'start': 1132.48,
   'text': 'banana dimensions. I know that a USDA identifies\xa0\n'
           'bananas as 7 to 8 in long. I need to run another\xa0\xa0'},
  {'duration': 4.64,
   'start': 1138.56,
   'text': 'web search. Let me convert these to more standard\xa0\n'
           "measurements. You can see it's using tool calls\xa0\xa0"},
  {'duration': 4.24,
   'start': 1143.2,
   'text': 'interled with thinking, which is something\xa0\n'
           'new that the quad 4 models can do. Finally,\xa0\xa0'},
  {'duration': 4.48,
   'start': 1147.44,
   'text': "it's running some calculations. It's about how\xa0\n"
           'many bananas could be packed into the cargo space\xa0\xa0'},
  {'duration': 16.64,
   'start': 1151.92,
   'text': "of the truck. And it's running a few more web\xa0\n"
           'searches. You can see here that this is a fairly'},
  {'duration': 3.04, 'start': 1168.56, 'text': 'pending'},
  {'duration': 5.92,
   'start': 1171.6,
   'text': "approximately 48,000 bananas. I've seen the model\xa0\n"
           'estimate anything between 30,000 50,000. I think\xa0\xa0'},
  {'duration': 8.4,
   'start': 1177.52,
   'text': 'the right answer is around 30,000. So this is this\xa0\n'
           'is roughly correct. Um going back to the slides,\xa0\xa0'},
  {'duration': 5.44,
   'start': 1185.92,
   'text': 'I think that you know this this sort of approach\xa0\n'
           'of testing out your prompt, seeing what tools\xa0\xa0'},
  {'duration': 4.4,
   'start': 1191.36,
   'text': 'the model calls, reading its thinking blocks,\xa0\n'
           "and actually seeing how the model's thinking\xa0\xa0"},
  {'duration': 5.36,
   'start': 1195.76,
   'text': 'will often make it really obvious. um what the\xa0\n'
           "issues are and what's going wrong. So you'll\xa0\xa0"},
  {'duration': 4.4,
   'start': 1201.12,
   'text': "test it out and you'll just see like okay\xa0\n"
           "maybe the model's using too many tools here,\xa0\xa0"},
  {'duration': 4.56,
   'start': 1205.52,
   'text': "maybe it's using the wrong sources or maybe\xa0\n"
           "it's just following the wrong guidelines. Um\xa0\xa0"},
  {'duration': 7.28,
   'start': 1210.08,
   'text': 'so this is a really helpful way to sort of think\xa0\n'
           'like your agents and make them more concrete.'},
  {'duration': 6.4,
   'start': 1217.36,
   'text': 'Um switching back to the slides.'},
  {'duration': 5.6,
   'start': 1223.76,
   'text': 'Okay, so eval evaluations are really important\xa0\n'
           "for any system. Um, they're really important\xa0\xa0"},
  {'duration': 4.88,
   'start': 1229.36,
   'text': "for systematically measuring whether you're\xa0\n"
           'making progress in your prompt. Very quickly,\xa0\xa0'},
  {'duration': 4.48,
   'start': 1234.24,
   'text': "you'll notice that it's difficult to really make\xa0\n"
           "progress on a prompt if you don't have an eval\xa0\xa0"},
  {'duration': 3.52,
   'start': 1238.72,
   'text': 'that tells you meaningfully whether your prompt is\xa0\n'
           'getting better and whether your system is getting\xa0\xa0'},
  {'duration': 6.4,
   'start': 1242.24,
   'text': 'better. But eval are much more difficult for\xa0\n'
           'agents. Um, agents are longunning. They do a bunch\xa0\xa0'},
  {'duration': 5.6,
   'start': 1248.64,
   'text': 'of things. They may not they may not always have\xa0\n'
           'a predictable process. classification is easier to\xa0\xa0'},
  {'duration': 5.2,
   'start': 1254.24,
   'text': 'eval because you can just check did it classify\xa0\n'
           'this output correctly but agents are harder. So\xa0\xa0'},
  {'duration': 5.2,
   'start': 1259.44,
   'text': 'a few tips to make this a bit easier. One is that\xa0\n'
           'the larger the effect size the smaller the sample\xa0\xa0'},
  {'duration': 5.04,
   'start': 1264.64,
   'text': 'size you mean you need um and so this is sort of\xa0\n'
           'just a principle from science in general where\xa0\xa0'},
  {'duration': 5.52,
   'start': 1269.68,
   'text': 'if an effect size is very large for example if a\xa0\n'
           "medication will cure people immediately you don't\xa0\xa0"},
  {'duration': 4.72,
   'start': 1275.2,
   'text': 'really need a large sample size of a ton of people\xa0\n'
           'to know that the model is that that this treatment\xa0\xa0'},
  {'duration': 4.88,
   'start': 1279.92,
   'text': 'is having an effect. Similarly, when you change a\xa0\n'
           "prompt, if it's really obvious that the system is\xa0\xa0"},
  {'duration': 4.48,
   'start': 1284.8,
   'text': "getting better, you don't need a large eval. I\xa0\n"
           'often see teams think that they need to set up\xa0\xa0'},
  {'duration': 4.4,
   'start': 1289.28,
   'text': 'a huge eval of like hundreds of test cases and\xa0\n'
           "make it completely automated when they're just\xa0\xa0"},
  {'duration': 4.72,
   'start': 1293.68,
   'text': 'starting out building an agent. This is a failure\xa0\n'
           "mode and it's an antiattern. You should start out\xa0\xa0"},
  {'duration': 5.84,
   'start': 1298.4,
   'text': 'with a very small eval and just run it and see\xa0\n'
           'what happens. You can even start out manually. Um,\xa0\xa0'},
  {'duration': 4.88,
   'start': 1304.24,
   'text': 'but the important thing is to just get started.\xa0\n'
           'I often see teams delaying evals because they\xa0\xa0'},
  {'duration': 4.72,
   'start': 1309.12,
   'text': "think that they're so intimidating or that they\xa0\n"
           'need such a sort of intense eval to really get\xa0\xa0'},
  {'duration': 4.32,
   'start': 1313.84,
   'text': 'some signal, but you can get great signal from\xa0\n'
           'a small number of test cases. You just want to\xa0\xa0'},
  {'duration': 4.56,
   'start': 1318.16,
   'text': 'keep those test cases s consistent and then keep\xa0\n'
           'testing them so you know whether the model and\xa0\xa0'},
  {'duration': 5.2,
   'start': 1322.72,
   'text': 'the prompt is getting better. You also want to\xa0\n'
           "use realistic tasks. So don't just sort of come\xa0\xa0"},
  {'duration': 4.64,
   'start': 1327.92,
   'text': 'up with arbitrary prompts or descriptions\xa0\n'
           "or tasks that don't really have any real\xa0\xa0"},
  {'duration': 4.32,
   'start': 1332.56,
   'text': 'correlation to what your system will be doing.\xa0\n'
           "For example, if you're working on coding tasks,\xa0\xa0"},
  {'duration': 4.24,
   'start': 1336.88,
   'text': "you don't won't want to give the model just\xa0\n"
           'competitive programming problems because this is\xa0\xa0'},
  {'duration': 4.72,
   'start': 1341.12,
   'text': "not what real world coding is like. You'll want to\xa0\n"
           'give it realistic tasks that really reflect what\xa0\xa0'},
  {'duration': 4.72,
   'start': 1345.84,
   'text': 'your agent will be doing. Similarly, in finance,\xa0\n'
           "you'll want to sort of take tasks that real people\xa0\xa0"},
  {'duration': 5.12,
   'start': 1350.56,
   'text': 'are trying to solve and just use them to evaluate\xa0\n'
           'whether the model can do those. This allows you\xa0\xa0'},
  {'duration': 4.72,
   'start': 1355.68,
   'text': 'to really measure whether the model is getting\xa0\n'
           'better at the tasks that you care about. Another\xa0\xa0'},
  {'duration': 5.04,
   'start': 1360.4,
   'text': 'point is that LLM is judge is really powerful,\xa0\n'
           'especially when you give it a rubric. So agents\xa0\xa0'},
  {'duration': 3.52,
   'start': 1365.44,
   'text': 'will have lots of different kinds of outputs.\xa0\n'
           "For example, if you're using them for search,\xa0\xa0"},
  {'duration': 4.96,
   'start': 1368.96,
   'text': 'they might have tons of different kinds of search\xa0\n'
           'reports with different kinds of structure. But LMS\xa0\xa0'},
  {'duration': 4.48,
   'start': 1373.92,
   'text': 'are great at handling lots of different kinds of\xa0\n'
           'structure and text with different characteristics.\xa0\xa0'},
  {'duration': 4.56,
   'start': 1378.4,
   'text': "And so one thing that we've done, for example,\xa0\n"
           'is given the model just a clear rubric and then\xa0\xa0'},
  {'duration': 4.64,
   'start': 1382.96,
   'text': 'ask it to evaluate the output of the agent.\xa0\n'
           'For example, for search tasks, we might give\xa0\xa0'},
  {'duration': 4.72,
   'start': 1387.6,
   'text': 'it a rubric that says, check that the model,\xa0\n'
           'you know, um, looked at the right sources,\xa0\xa0'},
  {'duration': 4.8,
   'start': 1392.32,
   'text': 'check that it got the correct answer. In this\xa0\n'
           'case, we might say, um, check that the model\xa0\xa0'},
  {'duration': 5.84,
   'start': 1397.12,
   'text': 'guessed that the amount of bananas that can fit\xa0\n'
           'in a Rivian R1s is between like 10,000 and 50,000.\xa0\xa0'},
  {'duration': 4.88,
   'start': 1402.96,
   'text': 'Anything outside that range is not realistic. So,\xa0\n'
           'you know, you can use things like that to sort of\xa0\xa0'},
  {'duration': 4.96,
   'start': 1407.84,
   'text': 'benchmark whether the model is getting the right\xa0\n'
           "answers, whether it's following the right process.\xa0\xa0"},
  {'duration': 4.16,
   'start': 1413.44,
   'text': 'At the end of the day though, nothing is a perfect\xa0\n'
           'replacement for human evals. You need to test the\xa0\xa0'},
  {'duration': 4.4,
   'start': 1417.6,
   'text': "system manually. You need to see what it's doing.\xa0\n"
           'You need to sort of look at the transcripts, look\xa0\xa0'},
  {'duration': 6.0,
   'start': 1422.0,
   'text': 'at what the model is doing, and sort of understand\xa0\n'
           'your system if you want to make progress on it.\xa0\xa0'},
  {'duration': 5.36,
   'start': 1428.0,
   'text': 'Here are some examples of eval. So one example\xa0\n'
           'that I sort of showed uh talked about is answer\xa0\xa0'},
  {'duration': 4.96,
   'start': 1433.36,
   'text': 'accuracy. And this is where you just use an LLM\xa0\n'
           'as judge to judge whether the answer is accurate.\xa0\xa0'},
  {'duration': 4.56,
   'start': 1438.32,
   'text': 'So for example in this case you might say the\xa0\n'
           'agent needs to use a tool to query the number\xa0\xa0'},
  {'duration': 4.08,
   'start': 1442.88,
   'text': 'of employees and then report the answer and then\xa0\n'
           'you know the number of employees at your company.\xa0\xa0'},
  {'duration': 4.4,
   'start': 1446.96,
   'text': 'So you can just check that with an LM as judge.\xa0\n'
           'The reason you use an LMS as judge here is because\xa0\xa0'},
  {'duration': 5.36,
   'start': 1451.36,
   'text': "it's more robust to variations. For example, if\xa0\n"
           "you're just checking for the integer 47 in this\xa0\xa0"},
  {'duration': 5.68,
   'start': 1456.72,
   'text': 'case in the output that is not very robust and\xa0\n'
           "if the model says 47 as text you'll grade it\xa0\xa0"},
  {'duration': 4.8,
   'start': 1462.4,
   'text': 'incorrectly. So you want to use an LMS as judge\xa0\n'
           'there to be robust to those minor variations.\xa0\xa0'},
  {'duration': 4.64,
   'start': 1467.2,
   'text': 'Another way you can eval agents is tool use\xa0\n'
           'accuracy. Agents involve using tools in a\xa0\xa0'},
  {'duration': 4.64,
   'start': 1471.84,
   'text': 'loop. And so if you know in advance what tools\xa0\n'
           'the model should use or how it should use them,\xa0\xa0'},
  {'duration': 5.2,
   'start': 1476.48,
   'text': 'you can just evaluate if it used the correct tools\xa0\n'
           'in the process. For example, in this case, I might\xa0\xa0'},
  {'duration': 6.0,
   'start': 1481.68,
   'text': 'evaluate the agent should use web search at least\xa0\n'
           'five times to answer this question. And so I could\xa0\xa0'},
  {'duration': 5.28,
   'start': 1487.68,
   'text': 'just check in the transcript programmatically did\xa0\n'
           'the tool call for web search appear five times or\xa0\xa0'},
  {'duration': 4.64,
   'start': 1492.96,
   'text': 'not. Similarly, you might check in this case\xa0\n'
           'for in response to the question book a flight,\xa0\xa0'},
  {'duration': 4.4,
   'start': 1497.6,
   'text': 'the agent should use the search flights tool\xa0\n'
           'and you can just check that programmatically\xa0\xa0'},
  {'duration': 4.8,
   'start': 1502.0,
   'text': 'and this allows you to make sure that the right\xa0\n'
           'tools are being used at the right times. Finally,\xa0\xa0'},
  {'duration': 5.28,
   'start': 1506.8,
   'text': 'a really good eval for agents is tobench. You\xa0\n'
           'can sort of look this up. Towen is a sort of open\xa0\xa0'},
  {'duration': 5.52,
   'start': 1512.08,
   'text': 'source benchmark that shows that you can evaluate\xa0\n'
           'whether agents reach the correct final state.\xa0\xa0'},
  {'duration': 5.6,
   'start': 1517.6,
   'text': 'So a lot of agents are sort of modifying a\xa0\n'
           'database or interacting with a user in a way\xa0\xa0'},
  {'duration': 5.2,
   'start': 1523.2,
   'text': 'where you can say the model should always get to\xa0\n'
           'this state at the end of the process. For example,\xa0\xa0'},
  {'duration': 6.88,
   'start': 1528.4,
   'text': 'if your agent is a customer service agent for\xa0\n'
           'airlines and the user asks to change their flight\xa0\xa0'},
  {'duration': 4.72,
   'start': 1535.28,
   'text': 'at the end of the agentic process in response to\xa0\n'
           'that prompt, it should have changed the flight in\xa0\xa0'},
  {'duration': 5.12,
   'start': 1540.0,
   'text': 'the database. And so you can just check at the end\xa0\n'
           'of the agentic process, was the flight changed?\xa0\xa0'},
  {'duration': 4.4,
   'start': 1545.12,
   'text': 'was this row in the database changed to a\xa0\n'
           'different date and that can verify that the\xa0\xa0'},
  {'duration': 4.4,
   'start': 1549.52,
   'text': 'agent is working correctly. This is really robust\xa0\n'
           'and you can use it a lot in a lot of different use\xa0\xa0'},
  {'duration': 4.72,
   'start': 1553.92,
   'text': 'cases. For example, you can check that your\xa0\n'
           'database is updated correctly. You can check\xa0\xa0'},
  {'duration': 4.72,
   'start': 1558.64,
   'text': 'that certain files were modified, things like\xa0\n'
           'that as a way to evaluate the final state that\xa0\xa0'},
  {'duration': 9.08,
   'start': 1563.36,
   'text': "the agent reaches. And that's it from us. Um,\xa0\n"
           "we're happy to take your questions. [Applause]"},
  {'duration': 4.48,
   'start': 1577.44,
   'text': 'Can you talk about building prompts for agents?\xa0\n'
           'Are you giving it kind of long longer prompts\xa0\xa0'},
  {'duration': 4.16,
   'start': 1581.92,
   'text': 'first and then iterating or you starting kind\xa0\n'
           "of chunk by chunk? Uh what's that look like?\xa0\xa0"},
  {'duration': 6.16,
   'start': 1586.08,
   'text': 'And can you show sort of a little bit more on that\xa0\n'
           "thought process? That's a great question. Um can\xa0\xa0"},
  {'duration': 5.28,
   'start': 1592.24,
   'text': 'I switch back to my screen actually? I just want\xa0\n'
           'to sort of show the demo. Thank you. Um, yeah. So,\xa0\xa0'},
  {'duration': 4.0,
   'start': 1597.52,
   'text': 'you can see this is sort of a final prompt\xa0\n'
           "that we've arrived at, but this is not where\xa0\xa0"},
  {'duration': 13.6,
   'start': 1601.52,
   'text': 'we started. I think the answer to your question\xa0\n'
           'is that you start with a short simple prompt.'},
  {'duration': 5.04,
   'start': 1615.12,
   'text': 'Um, and I might just say search the web\xa0\n'
           "aentically. I'll change this to a different\xa0\xa0"},
  {'duration': 6.88,
   'start': 1620.16,
   'text': 'question. Um, how good are the Cloud 4 models\xa0\n'
           "and then we'll just run that. And so you'll\xa0\xa0"},
  {'duration': 4.24,
   'start': 1627.04,
   'text': 'want to start with something very simple and just\xa0\n'
           "see how it works. You'll often find that Claude\xa0\xa0"},
  {'duration': 4.72,
   'start': 1631.28,
   'text': 'can do the task well out of the box. But if you\xa0\n'
           'have more needs and you need it to operate really\xa0\xa0'},
  {'duration': 5.12,
   'start': 1636.0,
   'text': "consistently in production, you'll notice edge\xa0\n"
           'cases or small flaws as you test with more use\xa0\xa0'},
  {'duration': 4.8,
   'start': 1641.12,
   'text': "cases. And so you'll sort of add those into the\xa0\n"
           'prompt. So I would say building an agent prompt\xa0\xa0'},
  {'duration': 6.48,
   'start': 1645.92,
   'text': 'what it looks like concretely is start simple,\xa0\n'
           'test it out, see what happens, iterate from there,\xa0\xa0'},
  {'duration': 5.36,
   'start': 1652.4,
   'text': 'start collecting test cases where the model fails\xa0\n'
           'or succeeds and then over time try to increase the\xa0\xa0'},
  {'duration': 4.4,
   'start': 1657.76,
   'text': 'number of test cases that pass. Um, and the way\xa0\n'
           'to do this is by sort of adding instructions,\xa0\xa0'},
  {'duration': 4.16,
   'start': 1662.16,
   'text': 'adding examples to the prompt. But you really\xa0\n'
           'only do that when you find out what the edge\xa0\xa0'},
  {'duration': 6.32,
   'start': 1666.32,
   'text': 'cases are. And you can see that it thinks that\xa0\n'
           "the models are indeed good. So that's great.\xa0\xa0"},
  {'duration': 4.32,
   'start': 1673.84,
   'text': "when I do like normal prompting and it's not\xa0\n"
           "agentic, uh I'll often give like a few shot\xa0\xa0"},
  {'duration': 3.28,
   'start': 1678.16,
   'text': "example of like, hey, here's like input,\xa0\n"
           "here's output. This works really well for\xa0\xa0"},
  {'duration': 4.88,
   'start': 1681.44,
   'text': 'like classification tasks, stuff like that, right?\xa0\n'
           'Uh is there a parallel here in this like agentic\xa0\xa0'},
  {'duration': 4.48,
   'start': 1686.32,
   'text': "world? Are you finding that that's ever helpful\xa0\n"
           'or should I not think about it that way? That is\xa0\xa0'},
  {'duration': 5.84,
   'start': 1690.8,
   'text': 'a great question. Yeah. So should you include\xa0\n'
           'fewshot examples in your prompt and sort of\xa0\xa0'},
  {'duration': 3.6,
   'start': 1696.64,
   'text': 'traditional prompting techniques involve like\xa0\n'
           'giving the saying the model should use a chain\xa0\xa0'},
  {'duration': 4.72,
   'start': 1700.24,
   'text': 'of thought and then giving a few shot examples\xa0\n'
           'like a bunch of examples to imitate. We find\xa0\xa0'},
  {'duration': 5.04,
   'start': 1704.96,
   'text': 'that these techniques are not as effective for\xa0\n'
           'state-of-the-art frontier models and for agents.\xa0\xa0'},
  {'duration': 4.24,
   'start': 1710.0,
   'text': 'Um the main reason for this is that if you give\xa0\n'
           'the model a bunch of examples of exactly what\xa0\xa0'},
  {'duration': 4.08,
   'start': 1714.24,
   'text': 'process it should follow, that just limits the\xa0\n'
           'model too much. These models are smarter than\xa0\xa0'},
  {'duration': 4.72,
   'start': 1718.32,
   'text': "you can predict and so you don't want to tell\xa0\n"
           'them exactly what they need to do. Similarly,\xa0\xa0'},
  {'duration': 3.92,
   'start': 1723.04,
   'text': 'chain of thought has just been trained into the\xa0\n'
           'models at this point. The models know to think\xa0\xa0'},
  {'duration': 5.84,
   'start': 1726.96,
   'text': "in advance. They don't need to be told like use\xa0\n"
           'chain of thought. But what we can do here is one\xa0\xa0'},
  {'duration': 4.24,
   'start': 1732.8,
   'text': 'you can tell the model how to use its thinking.\xa0\n'
           'So you know I talked about earlier rather than\xa0\xa0'},
  {'duration': 3.52,
   'start': 1737.04,
   'text': 'telling the model you need to use a chain of\xa0\n'
           'thought. It already knows that. You can just\xa0\xa0'},
  {'duration': 4.88,
   'start': 1740.56,
   'text': 'say use your thinking process to plan out your\xa0\n'
           "search or to plan out what you're going to do\xa0\xa0"},
  {'duration': 4.96,
   'start': 1745.44,
   'text': 'in terms of coding. Reme or you can tell it to\xa0\n'
           'remember specific things in its thinking process\xa0\xa0'},
  {'duration': 4.64,
   'start': 1750.4,
   'text': 'and that sort of helps the agent stay on track.\xa0\n'
           "As far as examples go, um you'll want to give\xa0\xa0"},
  {'duration': 4.24,
   'start': 1755.04,
   'text': 'the model examples but not too prescriptive.\xa0\n'
           'I think we are out of time, but you can come\xa0\xa0'},
  {'duration': 6.16,
   'start': 1759.28,
   'text': "up to me personally and I'll talk to you all\xa0\n"
           'after. Thanks. Thank you. Thanks for coming.'}]]
