[[{'duration': 5.44,
   'start': 5.04,
   'text': 'Hi everyone. Thank you for joining us today for\xa0\n'
           "prompting 101. Uh my name is Hannah. I'm part\xa0\xa0"},
  {'duration': 4.8,
   'start': 10.48,
   'text': 'of the applied AI team here at Anthropic. And\xa0\n'
           'with me is Christian, also part of the applied\xa0\xa0'},
  {'duration': 4.4,
   'start': 15.28,
   'text': "AI team. And what we're going to do today is\xa0\n"
           'take you through a little bit of prompting best\xa0\xa0'},
  {'duration': 5.76,
   'start': 19.68,
   'text': "practices. And we're going to use a real world\xa0\n"
           'scenario and build up a prompt together. Uh so\xa0\xa0'},
  {'duration': 5.04,
   'start': 25.44,
   'text': 'a little bit about what prompt engineering is. uh\xa0\n'
           "prompt engineering. You're all probably a little\xa0\xa0"},
  {'duration': 4.4,
   'start': 30.48,
   'text': 'bit familiar with this. This is the way that we\xa0\n'
           'communicate with a language model and try to get\xa0\xa0'},
  {'duration': 4.64,
   'start': 34.88,
   'text': 'it to do what we want. So, this is the practice of\xa0\n'
           'writing clear instructions for the model, giving\xa0\xa0'},
  {'duration': 4.8,
   'start': 39.52,
   'text': 'the model the context that it needs to complete\xa0\n'
           'the task, and thinking through how we want to\xa0\xa0'},
  {'duration': 5.04,
   'start': 44.32,
   'text': 'arrange that information in order to get the\xa0\n'
           "best result. Um, so there's a lot of detail here,\xa0\xa0"},
  {'duration': 4.4,
   'start': 49.36,
   'text': 'a lot of different ways you might want to think\xa0\n'
           'about building out a prompt. Um, and as always,\xa0\xa0'},
  {'duration': 4.4,
   'start': 53.76,
   'text': 'the best way to learn this is just to practice\xa0\n'
           "doing it. Um, so today we're going to go through\xa0\xa0"},
  {'duration': 5.68,
   'start': 58.16,
   'text': "a hands-on scenario. Uh, we're going to use an\xa0\n"
           'example inspired by a real customer that we worked\xa0\xa0'},
  {'duration': 4.64,
   'start': 63.84,
   'text': "with. So, we've modified what the actual customer\xa0\n"
           'asked us to do, but this is a really interesting\xa0\xa0'},
  {'duration': 5.6,
   'start': 68.48,
   'text': 'case of trying to analyze some images and get uh\xa0\n'
           'factual information out of the images and have\xa0\xa0'},
  {'duration': 6.32,
   'start': 74.08,
   'text': 'Claude make a judgment about what content it finds\xa0\n'
           'there. And I actually do not speak the language\xa0\xa0'},
  {'duration': 4.72,
   'start': 80.4,
   'text': 'that this content is in, but luckily Christian and\xa0\n'
           "Claude both do. Uh so I'm going to pass it over\xa0\xa0"},
  {'duration': 5.2,
   'start': 85.12,
   'text': 'to Christian to talk about the scenario and the\xa0\n'
           'content. So for this example that we have here,\xa0\xa0'},
  {'duration': 5.52,
   'start': 90.32,
   'text': "it's uh intended so so to set the stage, imagine\xa0\n"
           "you're working for a Swedish insurance company\xa0\xa0"},
  {'duration': 5.68,
   'start': 95.84,
   'text': 'and you deal with uh car insurance claims on a\xa0\n'
           'daily manner. Um and the purpose of this is that\xa0\xa0'},
  {'duration': 4.4,
   'start': 101.52,
   'text': "you have two pieces of information. Um we're going\xa0\n"
           'to these in detail as well, but visually you can\xa0\xa0'},
  {'duration': 6.56,
   'start': 105.92,
   'text': 'see on the left hand side we have a car accident\xa0\n'
           'report form. um just detailing out what transpired\xa0\xa0'},
  {'duration': 5.12,
   'start': 112.48,
   'text': 'before the action accident actually took place.\xa0\n'
           'And then finally, we have a sort of human drawn\xa0\xa0'},
  {'duration': 5.92,
   'start': 117.6,
   'text': 'um sketch of how the accident took place as well.\xa0\n'
           "So these two pieces of information is what we're\xa0\xa0"},
  {'duration': 4.88,
   'start': 123.52,
   'text': 'going to try to pass on to cloud. And to begin\xa0\n'
           'with, we could just take these two and throw them\xa0\xa0'},
  {'duration': 4.72,
   'start': 128.4,
   'text': 'into a console and just see what what happens.\xa0\n'
           'So if we transition over to console as well,\xa0\xa0'},
  {'duration': 4.56,
   'start': 133.12,
   'text': 'we can actually do this in a real manner. And\xa0\n'
           'in this case here, you can see we have our\xa0\xa0'},
  {'duration': 6.88,
   'start': 137.68,
   'text': "shiny beautiful entropic console. We're using the\xa0\n"
           'new claw for solid model as well. In this case,\xa0\xa0'},
  {'duration': 5.44,
   'start': 144.56,
   'text': 'setting temperature zero and having a a huge max\xa0\n'
           'token budget as well. Just helping us make sure\xa0\xa0'},
  {'duration': 4.96,
   'start': 150.0,
   'text': "that there's no limitations to what CL can do. In\xa0\n"
           'this case, you can see I have a very simple prompt\xa0\xa0'},
  {'duration': 4.0,
   'start': 154.96,
   'text': "just setting the stage of what Cloud's supposed\xa0\n"
           'to do. in this case mentioning that this is\xa0\xa0'},
  {'duration': 6.64,
   'start': 158.96,
   'text': 'um intend to review a an accident report form uh\xa0\n'
           'and eventually also determine um what happened\xa0\xa0'},
  {'duration': 4.16,
   'start': 165.6,
   'text': "in an accident and who's at fault. So you can\xa0\n"
           'see here with this very simple prompt if I just\xa0\xa0'},
  {'duration': 9.2,
   'start': 169.76,
   'text': 'run this let me go to preview. Uh we can see here\xa0\n'
           'that Claude thinks that this is in relation to a\xa0\xa0'},
  {'duration': 6.08,
   'start': 178.96,
   'text': 'skiing accident that happened on a street called\xa0\n'
           "Chappangan. It's a very common street in Sweden.\xa0\xa0"},
  {'duration': 4.96,
   'start': 185.04,
   'text': 'Um and in many ways you can sort of understand\xa0\n'
           'this innocent mistake in the sense that in our\xa0\xa0'},
  {'duration': 5.36,
   'start': 190.0,
   'text': "prompt we actually haven't done anything to set\xa0\n"
           'the stage on what is actually taking place here.\xa0\xa0'},
  {'duration': 4.8,
   'start': 195.36,
   'text': 'So this sort of first guess is not too bad but\xa0\n'
           'we still notice a lot of intuition that we can\xa0\xa0'},
  {'duration': 7.2,
   'start': 200.16,
   'text': 'bake into cloud. So if we switch back to the\xa0\n'
           'slides you can see here that um in many ways\xa0\xa0'},
  {'duration': 4.88,
   'start': 207.36,
   'text': 'prompt engineering is a very iterative empirical\xa0\n'
           'science. Uh in this case here, we could almost\xa0\xa0'},
  {'duration': 5.92,
   'start': 212.24,
   'text': 'have a test case where Claude is supposed to make\xa0\n'
           "sure it understands it's in a car or vehicular\xa0\xa0"},
  {'duration': 5.6,
   'start': 218.16,
   'text': 'environment, nothing to do with skiing. Uh and in\xa0\n'
           'that way, you iteratively build upon your prompt\xa0\xa0'},
  {'duration': 4.8,
   'start': 223.76,
   'text': "to make sure it's actually tackling the problem\xa0\n"
           "you're intending to solve. Um and to do so,\xa0\xa0"},
  {'duration': 5.76,
   'start': 228.56,
   'text': "we'll go through some best practices of how we we\xa0\n"
           'at Anthropic break this down internally and how we\xa0\xa0'},
  {'duration': 4.72,
   'start': 234.32,
   'text': "recommend others to do so as well. So, we're going\xa0\n"
           'to talk about some best practices for developing\xa0\xa0'},
  {'duration': 4.96,
   'start': 239.04,
   'text': 'a great prompt. Uh, first we want to talk a\xa0\n'
           'little bit about what a great prompt structure\xa0\xa0'},
  {'duration': 5.12,
   'start': 244.0,
   'text': 'looks like. So you might be familiar with kind of\xa0\n'
           'interacting with a chatbot with Claude going back\xa0\xa0'},
  {'duration': 4.88,
   'start': 249.12,
   'text': 'and forth having a more kind of conversational\xa0\n'
           "style interaction. When we're working with a task\xa0\xa0"},
  {'duration': 5.12,
   'start': 254.0,
   'text': "like this, we're probably using the API and we\xa0\n"
           'kind of want to send one single message to Claude\xa0\xa0'},
  {'duration': 5.68,
   'start': 259.12,
   'text': 'and have it nail the task the first time around\xa0\n'
           'without needing to uh kind of move back and forth.\xa0\xa0'},
  {'duration': 5.28,
   'start': 264.8,
   'text': 'Uh, so the kind of structure that we recommend is\xa0\n'
           'setting the task description up front. So telling\xa0\xa0'},
  {'duration': 3.92,
   'start': 270.08,
   'text': 'Claude, "What are you here to do? What\'s your\xa0\n'
           'role? What task are you trying to accomplish\xa0\xa0'},
  {'duration': 4.24,
   'start': 274.0,
   'text': 'today?" Then we provide content. So in this\xa0\n'
           "case, it's the images that Christian was showing,\xa0\xa0"},
  {'duration': 5.6,
   'start': 278.24,
   'text': 'the form and the drawing of the accident and how\xa0\n'
           "they occurred. That's our dynamic content. This\xa0\xa0"},
  {'duration': 3.52,
   'start': 283.84,
   'text': "might also be something you're retrieving from\xa0\n"
           'another system, depending on what your use case\xa0\xa0'},
  {'duration': 5.36,
   'start': 287.36,
   'text': "is. We're going to give some detailed instructions\xa0\n"
           'to Claude, so almost like a step-by-step list of\xa0\xa0'},
  {'duration': 5.6,
   'start': 292.72,
   'text': 'how we want Claude to go through the task and how\xa0\n'
           'we want it to um tackle the reasoning. We may give\xa0\xa0'},
  {'duration': 4.96,
   'start': 298.32,
   'text': "some examples to Claude. Here's an example of some\xa0\n"
           "piece of content you might receive. Here's how you\xa0\xa0"},
  {'duration': 4.72,
   'start': 303.28,
   'text': 'should respond when given that content. And at\xa0\n'
           'the end, we usually recommend repeating anything\xa0\xa0'},
  {'duration': 4.8,
   'start': 308.0,
   'text': "that's really important for Claude to understand\xa0\n"
           'about this task. Kind of uh reviewing the\xa0\xa0'},
  {'duration': 4.8,
   'start': 312.8,
   'text': 'information with Claude, emphasizing things that\xa0\n'
           'are extra critical and then telling Claude, "Okay,\xa0\xa0'},
  {'duration': 5.52,
   'start': 317.6,
   'text': 'go ahead and do your work." So, here\'s another\xa0\n'
           'view. This has a little bit more detail, a little\xa0\xa0'},
  {'duration': 4.72,
   'start': 323.12,
   'text': "bit more of a breakdown, and we're going to walk\xa0\n"
           'through each of these 10 points individually and\xa0\xa0'},
  {'duration': 5.2,
   'start': 327.84,
   'text': 'show you how we build this up, um, in the console.\xa0\n'
           "So, the first couple things, um, Christian's\xa0\xa0"},
  {'duration': 5.68,
   'start': 333.04,
   'text': 'going to talk about the task context and the tone\xa0\n'
           'context. Perfect. So, yeah, if we begin with the\xa0\xa0'},
  {'duration': 4.8,
   'start': 338.72,
   'text': 'task context, as you realized when I went through\xa0\n'
           "a little demo there, um, we didn't have much\xa0\xa0"},
  {'duration': 5.44,
   'start': 343.52,
   'text': 'elaborating what what scenario Chlo was actually\xa0\n'
           'working within. And because of that, you can also\xa0\xa0'},
  {'duration': 4.24,
   'start': 348.96,
   'text': "tell that claw doesn't necessarily need to guess a\xa0\n"
           'lot more on what you actually want from it. So in\xa0\xa0'},
  {'duration': 4.0,
   'start': 353.2,
   'text': 'our case, we really want to break that down, make\xa0\n'
           'sure we can give more clear-cut instructions. Um,\xa0\xa0'},
  {'duration': 5.68,
   'start': 357.2,
   'text': "and also make sure we understand what's the\xa0\n"
           "task that we're asking Claw to do. Um, secondly,\xa0\xa0"},
  {'duration': 5.28,
   'start': 362.88,
   'text': 'as well, we also make sure we add a little bit of\xa0\n'
           'tone into it all. Um, key thing here is we want\xa0\xa0'},
  {'duration': 6.32,
   'start': 368.16,
   'text': 'Claw to stay factual and to stay confident. So if\xa0\n'
           "uh, Claw can't understand what it's looking at,\xa0\xa0"},
  {'duration': 4.56,
   'start': 374.48,
   'text': "we don't want to guess and just sort of mislead\xa0\n"
           'us. We want to make sure that any assessment\xa0\xa0'},
  {'duration': 4.32,
   'start': 379.04,
   'text': 'and in our case we want to make sure that we can\xa0\n'
           "understand who's at fault here. We want to make\xa0\xa0"},
  {'duration': 4.56,
   'start': 383.36,
   'text': 'sure that assessment is as clear and as confident\xa0\n'
           "as possible. If not, we're sort of losing track of\xa0\xa0"},
  {'duration': 6.48,
   'start': 387.92,
   'text': "what we're doing. So if we transition back to the\xa0\n"
           'the console, um we can jump to a V2 that we have\xa0\xa0'},
  {'duration': 7.36,
   'start': 394.4,
   'text': "here. So I'll just navigate to V2. And you can see\xa0\n"
           "here um I'll also just illustrate the data because\xa0\xa0"},
  {'duration': 3.36,
   'start': 401.76,
   'text': "we didn't really do that last time around just\xa0\n"
           "to really highlight what we're looking at. So,\xa0\xa0"},
  {'duration': 6.4,
   'start': 405.12,
   'text': "what we're seeing here, this is the car accident\xa0\n"
           "report form, and it's just 17 different checkboxes\xa0\xa0"},
  {'duration': 3.84,
   'start': 411.52,
   'text': 'going through what actually happened. You\xa0\n'
           "can see there's a vehicle A and vehicle B,\xa0\xa0"},
  {'duration': 3.52,
   'start': 415.36,
   'text': 'both on the left and right hand side. And the main\xa0\n'
           'purpose of this is that we want to make sure that\xa0\xa0'},
  {'duration': 6.96,
   'start': 418.88,
   'text': 'Claude can understand this manually generated data\xa0\n'
           "to assess what's actually going on. And that is\xa0\xa0"},
  {'duration': 5.76,
   'start': 425.84,
   'text': 'uh corroborated by if I navigate back here to this\xa0\n'
           'sketch that we can highlight here as well. In this\xa0\xa0'},
  {'duration': 6.64,
   'start': 431.6,
   'text': 'case, the form is just a different um data point\xa0\n'
           'for the same scenario. Um and in this case here, I\xa0\xa0'},
  {'duration': 5.84,
   'start': 438.24,
   'text': 'want to bake in more information into our version\xa0\n'
           "two. Uh and by doing so, I'm actually elaborating\xa0\xa0"},
  {'duration': 5.12,
   'start': 444.08,
   'text': "a lot more on what's going on. So, you can see\xa0\n"
           "here I'm specifying that uh this AI assistant is\xa0\xa0"},
  {'duration': 5.36,
   'start': 449.2,
   'text': "supposed to help a human's claim claims adjuster\xa0\n"
           "that's reviewing car accident report forms in\xa0\xa0"},
  {'duration': 4.56,
   'start': 454.56,
   'text': "Swedish as well. Um, you can see here we're also\xa0\n"
           "elaborating that it's a human-driven sketch of\xa0\xa0"},
  {'duration': 6.0,
   'start': 459.12,
   'text': 'the incident and that you should not um make an\xa0\n'
           "assessment if it's not actually fully confident.\xa0\xa0"},
  {'duration': 4.4,
   'start': 465.12,
   'text': "And that's really key because if we run this,\xa0\n"
           "you'll see that and you can see it's the same\xa0\xa0"},
  {'duration': 5.44,
   'start': 469.52,
   'text': 'settings as well. Clo my new shiny model zero\xa0\n'
           'temperature as well. If we run this, we can see\xa0\xa0'},
  {'duration': 7.52,
   'start': 474.96,
   'text': 'here what actually happens in this case. Um, CL is\xa0\n'
           "able to pick up that uh now it's relating to car\xa0\xa0"},
  {'duration': 5.52,
   'start': 482.48,
   'text': 'accidents, not skiing accidents, which is great.\xa0\n'
           "We can see it's able to pick up that vehicle A was\xa0\xa0"},
  {'duration': 5.76,
   'start': 488.0,
   'text': 'marked on on checkbox one and then vehicle B was\xa0\n'
           'on 12. Um, and if we scroll down though, we can\xa0\xa0'},
  {'duration': 5.52,
   'start': 493.76,
   'text': "still tell that there's some information missing\xa0\n"
           'for claw to make a fully confident determination\xa0\xa0'},
  {'duration': 4.72,
   'start': 499.28,
   'text': "of who's at fault here. And this is great. This\xa0\n"
           "is pertaining to a task set. Make sure you don't\xa0\xa0"},
  {'duration': 6.0,
   'start': 504.0,
   'text': "make anything any claims that aren't um uh factual\xa0\n"
           'and make sure you you only sort of assess things\xa0\xa0'},
  {'duration': 3.84,
   'start': 510.0,
   'text': "when you're when you're confident. But there's\xa0\n"
           "a lot of information we're still missing here.\xa0\xa0"},
  {'duration': 5.68,
   'start': 513.84,
   'text': 'um regarding the form uh what the form actually\xa0\n'
           'entails and a lot of that information is what\xa0\xa0'},
  {'duration': 5.12,
   'start': 519.52,
   'text': 'we want to want to bake into this LM application\xa0\n'
           'as well and the best way of doing so is actually\xa0\xa0'},
  {'duration': 6.08,
   'start': 524.64,
   'text': 'adding it to the system prompt which Hannah will\xa0\n'
           'elaborate on. Um so back in the slides uh we have\xa0\xa0'},
  {'duration': 6.0,
   'start': 530.72,
   'text': "the next item we're going to add to the prompt\xa0\n"
           'and this is um background detail data documents\xa0\xa0'},
  {'duration': 4.88,
   'start': 536.72,
   'text': 'and images and here as Christian was saying we\xa0\n'
           'actually know a lot about this form. the form is\xa0\xa0'},
  {'duration': 4.4,
   'start': 541.6,
   'text': 'going to be the same every single time. The form\xa0\n'
           'will never change. And so this is a really great\xa0\xa0'},
  {'duration': 4.56,
   'start': 546.0,
   'text': 'type of information to provide to Claude to tell\xa0\n'
           "Claude, here's the structure of the form you'll\xa0\xa0"},
  {'duration': 5.36,
   'start': 550.56,
   'text': 'be looking at. We know that will not ever alter\xa0\n'
           'between different queries. The way the form is\xa0\xa0'},
  {'duration': 4.96,
   'start': 555.92,
   'text': 'filled out will change, but the form itself is not\xa0\n'
           'going to change. And so this is a great type of\xa0\xa0'},
  {'duration': 4.4,
   'start': 560.88,
   'text': 'um information to put into the system prompt. Also\xa0\n'
           "a great thing to use prompt caching for if you're\xa0\xa0"},
  {'duration': 4.24,
   'start': 565.28,
   'text': 'considering using prompt caching. This will always\xa0\n'
           'be the same. And what this will help Claude do is\xa0\xa0'},
  {'duration': 5.84,
   'start': 569.52,
   'text': 'spend less time trying to figure out what the form\xa0\n'
           'is the first time it sees the form each time. And\xa0\xa0'},
  {'duration': 6.16,
   'start': 575.36,
   'text': "it's going to do a better job of reading the form\xa0\n"
           'because it already knows um what to expect there.\xa0\xa0'},
  {'duration': 5.76,
   'start': 581.52,
   'text': 'So another thing I want to touch on here is how we\xa0\n'
           'like to organize information in prompts. So Claude\xa0\xa0'},
  {'duration': 4.4,
   'start': 587.28,
   'text': 'really loves structure, loves organization.\xa0\n'
           "That's why we recommend following kind of a\xa0\xa0"},
  {'duration': 4.96,
   'start': 591.68,
   'text': "standard structure in your prompts. And there's\xa0\n"
           'a couple other tools you can use to help Claude\xa0\xa0'},
  {'duration': 4.8,
   'start': 596.64,
   'text': 'understand the information better. I also just\xa0\n'
           'want to mention all of this is in our docs with a\xa0\xa0'},
  {'duration': 4.32,
   'start': 601.44,
   'text': 'lot of really great examples. So definitely take\xa0\n'
           'pictures, but if you forget to take a picture,\xa0\xa0'},
  {'duration': 4.88,
   'start': 605.76,
   'text': "don't worry. All of this content is online with\xa0\n"
           'lots of examples and definitely encourage you\xa0\xa0'},
  {'duration': 7.28,
   'start': 610.64,
   'text': 'guys to check it out there too. Um anyway the uh\xa0\n'
           'so some things you can use delimiters like XML\xa0\xa0'},
  {'duration': 5.2,
   'start': 617.92,
   'text': 'tags also markdown is pretty useful to Claude\xa0\n'
           'but XML tags are nice because you can actually\xa0\xa0'},
  {'duration': 6.08,
   'start': 623.12,
   'text': "specify what's inside those tags. So we can tell\xa0\n"
           "Claude here's here's user preferences. Now you're\xa0\xa0"},
  {'duration': 3.84,
   'start': 629.2,
   'text': 'going to read some content and these XML tags are\xa0\n'
           'letting you know that everything wrapped in those\xa0\xa0'},
  {'duration': 5.04,
   'start': 633.04,
   'text': "tags is related to the user's preferences and\xa0\n"
           'it helps Claude refer back to that information\xa0\xa0'},
  {'duration': 6.48,
   'start': 638.08,
   'text': 'maybe at later points in the prompt. Um, so I\xa0\n'
           'want to show in the back in the console how we\xa0\xa0'},
  {'duration': 5.28,
   'start': 644.56,
   'text': "actually do this in this case. And Christian's\xa0\n"
           "going to pull up our version three. So we're\xa0\xa0"},
  {'duration': 4.96,
   'start': 649.84,
   'text': 'keeping everything about the other part of the\xa0\n'
           "user prompt the same. And we've decided in this\xa0\xa0"},
  {'duration': 4.48,
   'start': 654.8,
   'text': 'case to put this information in the system prompt.\xa0\n'
           "You could try this different ways. Uh, we're doing\xa0\xa0"},
  {'duration': 4.0,
   'start': 659.28,
   'text': "it in the system prompt here. And we're going\xa0\n"
           'to tell Claude everything it needs to know about\xa0\xa0'},
  {'duration': 4.48,
   'start': 663.28,
   'text': 'this form. So this is a Swedish car accident\xa0\n'
           "form. The form will be in Swedish. It'll have\xa0\xa0"},
  {'duration': 5.36,
   'start': 667.76,
   'text': "this title. It'll have two columns. The columns\xa0\n"
           "represent different vehicles. We'll tell Claude\xa0\xa0"},
  {'duration': 5.28,
   'start': 673.12,
   'text': 'about each of the 17 rows and what they mean.\xa0\n'
           'You might have noticed when we ran it before,\xa0\xa0'},
  {'duration': 5.44,
   'start': 678.4,
   'text': 'Claude was reading individually each of the lines\xa0\n'
           'to understand what they are. We can provide all of\xa0\xa0'},
  {'duration': 3.92,
   'start': 683.84,
   'text': "that information up front. And we're also going\xa0\n"
           'to give Claude a little bit of information about\xa0\xa0'},
  {'duration': 5.12,
   'start': 687.76,
   'text': 'how this form should be filled out. This is also\xa0\n'
           'really useful for Claude. We can tell it things\xa0\xa0'},
  {'duration': 4.24,
   'start': 692.88,
   'text': 'like, you know, humans are filling this form\xa0\n'
           "out basically. So, it's not going to be perfect.\xa0\xa0"},
  {'duration': 5.04,
   'start': 697.12,
   'text': 'People might put a circle. They might scribble.\xa0\n'
           'They might not put an X in the box. There could\xa0\xa0'},
  {'duration': 5.52,
   'start': 702.16,
   'text': 'be many types of markings that you need to look\xa0\n'
           "for when you're reading this form. Uh we can also\xa0\xa0"},
  {'duration': 3.84,
   'start': 707.68,
   'text': 'give Claude a little bit of information about how\xa0\n'
           'to interpret this or what the purpose or meaning\xa0\xa0'},
  {'duration': 5.2,
   'start': 711.52,
   'text': 'of this form is. And all of this is context\xa0\n'
           'that is hopefully really going to help Claude\xa0\xa0'},
  {'duration': 5.84,
   'start': 716.72,
   'text': 'um do a better job analyzing the form. So if\xa0\n'
           'we run it, everything else is still the same.\xa0\xa0'},
  {'duration': 6.32,
   'start': 722.56,
   'text': "So we've kept the same user prompt down here.\xa0\n"
           'Oh, your scroll is backwards from mine. Uh,\xa0\xa0'},
  {'duration': 5.68,
   'start': 728.88,
   'text': 'the we have the same user prompt here. Still\xa0\n'
           'asking Claude to do the same task, same context.\xa0\xa0'},
  {'duration': 4.8,
   'start': 734.56,
   'text': "And we'll see here that it's spending less time.\xa0\n"
           "It's kind of narrating to us a little bit less\xa0\xa0"},
  {'duration': 4.32,
   'start': 739.36,
   'text': 'about what the form is because it already knows\xa0\n'
           "what that is. And it's not concerned with kind\xa0\xa0"},
  {'duration': 4.96,
   'start': 743.68,
   'text': "of bringing us that information back. It's going\xa0\n"
           'to give us a whole list of what it found to be\xa0\xa0'},
  {'duration': 5.04,
   'start': 748.64,
   'text': 'checked, what the sketch shows. And here Claude\xa0\n'
           'is now becoming much more confident with this\xa0\xa0'},
  {'duration': 5.84,
   'start': 753.68,
   'text': 'additional context that we gave to Claude. Claude\xa0\n'
           "now feels it's appropriate to say vehicle B was\xa0\xa0"},
  {'duration': 4.8,
   'start': 759.52,
   'text': 'at fault in this case based on this drawing and\xa0\n'
           "based on this sketch. So already we're seeing some\xa0\xa0"},
  {'duration': 5.12,
   'start': 764.32,
   'text': 'improvement in the way Claude is analyzing these.\xa0\n'
           'I think we could probably all agree if we looked\xa0\xa0'},
  {'duration': 5.04,
   'start': 769.44,
   'text': 'at the drawing and at the list that vehicle\xa0\n'
           "B is at fault. Um so we'd like to see that.\xa0\xa0"},
  {'duration': 4.88,
   'start': 775.04,
   'text': "Uh so we're going to go back to the slides and\xa0\n"
           "talk about a couple of other items that we're not\xa0\xa0"},
  {'duration': 6.08,
   'start': 779.92,
   'text': 'really using in this prompt um but can be really\xa0\n'
           'helpful to building up uh building up your prompt\xa0\xa0'},
  {'duration': 5.52,
   'start': 786.0,
   'text': 'and making it work better. Exactly. I think um\xa0\n'
           'one thing that we really highlight is examples.\xa0\xa0'},
  {'duration': 7.2,
   'start': 791.52,
   'text': 'I think examples or few shot is a mechanism that\xa0\n'
           'really is powerful in steering cloud. So you can\xa0\xa0'},
  {'duration': 6.4,
   'start': 798.72,
   'text': 'imagine this um in in quite a non-trivial way as\xa0\n'
           'well. So imagine you have scenarios, situations,\xa0\xa0'},
  {'duration': 6.0,
   'start': 805.12,
   'text': 'even in this case concrete accidents that have\xa0\n'
           'happened that are um tricky for claw to get right.\xa0\xa0'},
  {'duration': 6.32,
   'start': 811.12,
   'text': 'But you with your human intuition and your human\xa0\n'
           'label data um is able to actually get to the right\xa0\xa0'},
  {'duration': 5.12,
   'start': 817.44,
   'text': 'conclusion. Then you can bake that information\xa0\n'
           'into the system problem itself by having clear-cut\xa0\xa0'},
  {'duration': 4.64,
   'start': 822.56,
   'text': "examples of a the data that that it's supposed\xa0\n"
           'to look at. So you can have visual examples.\xa0\xa0'},
  {'duration': 6.48,
   'start': 827.2,
   'text': 'you can just base 64 encode a a an image and have\xa0\n'
           "that as part of the data that you're passing along\xa0\xa0"},
  {'duration': 4.4,
   'start': 833.68,
   'text': 'into the examples and then on top of that you can\xa0\n'
           'have the sort of depiction or description rather\xa0\xa0'},
  {'duration': 4.72,
   'start': 838.08,
   'text': 'of how to break that down and understand it. This\xa0\n'
           'is something we really highlight and and emphasize\xa0\xa0'},
  {'duration': 5.68,
   'start': 842.8,
   'text': 'in how you can sort of push the limits of your\xa0\n'
           'LLM application is by baking in these examples\xa0\xa0'},
  {'duration': 4.24,
   'start': 848.48,
   'text': 'into system prompt. And this again is sort of the\xa0\n'
           'empirical science of prompt engineering that you\xa0\xa0'},
  {'duration': 4.4,
   'start': 852.72,
   'text': 'sort of always want to push the limits of your\xa0\n'
           'application and get that feedback loop in where\xa0\xa0'},
  {'duration': 4.64,
   'start': 857.12,
   'text': "it's going wrong and try to add that into system\xa0\n"
           'prompt so that next time when example that sort\xa0\xa0'},
  {'duration': 6.0,
   'start': 861.76,
   'text': "of mimics that u takes place it's able to actually\xa0\n"
           'reference it in its example set. You can see here\xa0\xa0'},
  {'duration': 6.24,
   'start': 867.76,
   'text': 'as well, this is just a little example of how we\xa0\n'
           'do this. Again, really emphasizing the sort of XML\xa0\xa0'},
  {'duration': 4.88,
   'start': 874.0,
   'text': "structure that we we um we enjoy. It's it gives a\xa0\n"
           "lot of structure to the clone. It's what it's been\xa0\xa0"},
  {'duration': 4.4,
   'start': 878.88,
   'text': 'fine-tuned on as well. Um and it works perfectly\xa0\n'
           "well for this example. And in our case, we're not\xa0\xa0"},
  {'duration': 4.0,
   'start': 883.28,
   'text': "doing this just because it's a simple demo,\xa0\n"
           'but you can realistically imagine if you were\xa0\xa0'},
  {'duration': 5.12,
   'start': 887.28,
   'text': "building this for an insurance company, you'd have\xa0\n"
           'tens, maybe even hundreds of examples are quite\xa0\xa0'},
  {'duration': 4.88,
   'start': 892.4,
   'text': "difficult, maybe in the gray, that you'd like to\xa0\n"
           'make sure that Claude actually has some basis in\xa0\xa0'},
  {'duration': 5.44,
   'start': 897.28,
   'text': 'to make the verdict next time. Um, another topic\xa0\n'
           "we really want to highlight, which we're not doing\xa0\xa0"},
  {'duration': 5.92,
   'start': 902.72,
   'text': "in this demo, is conversation history. It's in the\xa0\n"
           'same vein as examples. uh we use this to make sure\xa0\xa0'},
  {'duration': 7.04,
   'start': 908.64,
   'text': 'that the enough context rich information is at\xa0\n'
           'close disposal when it when when closing on on on\xa0\xa0'},
  {'duration': 5.84,
   'start': 915.68,
   'text': "your behalf. Um in our case now this isn't really\xa0\n"
           "a userfacing LLM application. It's more something\xa0\xa0"},
  {'duration': 3.76,
   'start': 921.52,
   'text': 'happening in the background. You can imagine for\xa0\n'
           'this insurance company they have this automated\xa0\xa0'},
  {'duration': 4.24,
   'start': 925.28,
   'text': 'system some data is generated out of this and then\xa0\n'
           'you might have a human in the loop at towards the\xa0\xa0'},
  {'duration': 5.28,
   'start': 929.52,
   'text': 'end. If you were have to build something much more\xa0\n'
           "userf facing where you'd have a long conversation\xa0\xa0"},
  {'duration': 5.76,
   'start': 934.8,
   'text': 'history that would be um relevant to bring in this\xa0\n'
           'is a perfect place in the system prompt to include\xa0\xa0'},
  {'duration': 7.2,
   'start': 940.56,
   'text': 'because it enriches the context that Claude works\xa0\n'
           "within. Um in our case we haven't done so but what\xa0\xa0"},
  {'duration': 6.96,
   'start': 947.76,
   'text': 'we do is and the next step is try to make sure\xa0\n'
           'we give a concrete reminder of the task at hand.\xa0\xa0'},
  {'duration': 3.44,
   'start': 955.76,
   'text': "So, now we're going to build out the\xa0\n"
           'final part of this prompt for Claude,\xa0\xa0'},
  {'duration': 4.56,
   'start': 959.2,
   'text': "and that's coming back to the reminder of what\xa0\n"
           'the immediate task is and giving Claude a reminder\xa0\xa0'},
  {'duration': 5.44,
   'start': 963.76,
   'text': 'about any important guidelines that we want it\xa0\n'
           'to follow. Some reasons that we may do this are\xa0\xa0'},
  {'duration': 7.44,
   'start': 969.2,
   'text': 'a preventing hallucinations. Um, so we want Claude\xa0\n'
           "to uh not invent details that it's not finding in\xa0\xa0"},
  {'duration': 4.56,
   'start': 976.64,
   'text': 'this prompt, right? Or not finding in the data.\xa0\n'
           "If Claude can't tell which form is checked,\xa0\xa0"},
  {'duration': 5.36,
   'start': 981.2,
   'text': "we don't want Claude to take its best guess or\xa0\n"
           'invent the idea that a box might be checked when\xa0\xa0'},
  {'duration': 5.68,
   'start': 986.56,
   'text': "it's not. If the sketch is unintelligible, the\xa0\n"
           'person did a really bad job drawing this drawing\xa0\xa0'},
  {'duration': 4.08,
   'start': 992.24,
   'text': 'and even a human would not be able to figure it\xa0\n'
           'out. We want Claude to be able to say that. And\xa0\xa0'},
  {'duration': 5.04,
   'start': 996.32,
   'text': "so these are some of the things we'll include in\xa0\n"
           'this final reminder and kind of wrap up step for\xa0\xa0'},
  {'duration': 5.04,
   'start': 1001.36,
   'text': 'Claude. Uh remind it to do things like answer only\xa0\n'
           "if it's very confident. We could even ask it to\xa0\xa0"},
  {'duration': 5.6,
   'start': 1006.4,
   'text': 'refer back to what it has seen in the form anytime\xa0\n'
           "it's making a factual claim. So if it wants to say\xa0\xa0"},
  {'duration': 5.76,
   'start': 1012.0,
   'text': 'vehicle B turned right, it should say I know this\xa0\n'
           'based on the fact that box two is clearly checked\xa0\xa0'},
  {'duration': 4.16,
   'start': 1017.76,
   'text': 'or whatever it might be. We can kind of give\xa0\n'
           'Claude some guidelines about that. So if we go\xa0\xa0'},
  {'duration': 7.76,
   'start': 1021.92,
   'text': 'back to the console, we can see the next version\xa0\n'
           "of the prompt and we're going to keep uh we're\xa0\xa0"},
  {'duration': 3.68,
   'start': 1029.68,
   'text': 'going to keep everything the same here in the\xa0\n'
           "system prompt. So, we're not changing any of that\xa0\xa0"},
  {'duration': 3.84,
   'start': 1033.36,
   'text': 'background context that we gave to Claude about\xa0\n'
           "the form, about how it's going to fill everything\xa0\xa0"},
  {'duration': 4.8,
   'start': 1037.2,
   'text': "out. We're not changing anything else about the\xa0\n"
           "context and the role. We're just adding this\xa0\xa0"},
  {'duration': 5.12,
   'start': 1042.0,
   'text': 'detailed list of tasks. And this is how we want\xa0\n'
           'Claude to go about analyzing this. And a really\xa0\xa0'},
  {'duration': 4.32,
   'start': 1047.12,
   'text': 'key thing that we found here as we were building\xa0\n'
           'this demo and when we were working on the customer\xa0\xa0'},
  {'duration': 5.44,
   'start': 1051.44,
   'text': 'example is that the order in which Claude analyzes\xa0\n'
           'this information is very important. And this is\xa0\xa0'},
  {'duration': 4.4,
   'start': 1056.88,
   'text': 'analogous to way you might think about doing this.\xa0\n'
           'If you were a human, you would probably not look\xa0\xa0'},
  {'duration': 4.4,
   'start': 1061.28,
   'text': 'at the drawing first and try to understand what\xa0\n'
           "was going on, right? It's pretty unclear. It's\xa0\xa0"},
  {'duration': 4.8,
   'start': 1065.68,
   'text': "a bunch of boxes and lines. We don't really know\xa0\n"
           'what that drawing is supposed to mean without any\xa0\xa0'},
  {'duration': 4.8,
   'start': 1070.48,
   'text': 'additional context. But if we have the form and we\xa0\n'
           "can read the form first and understand that we're\xa0\xa0"},
  {'duration': 4.64,
   'start': 1075.28,
   'text': "talking about a car accident and that we're seeing\xa0\n"
           "some checkboxes that indicate what vehicles we're\xa0\xa0"},
  {'duration': 4.48,
   'start': 1079.92,
   'text': 'doing at certain times, then we know a little\xa0\n'
           'bit more about how to understand what might be\xa0\xa0'},
  {'duration': 3.84,
   'start': 1084.4,
   'text': "in the drawing. And so that's the kind of detail\xa0\n"
           "that we're going to give Claude here is to say,\xa0\xa0"},
  {'duration': 4.96,
   'start': 1088.24,
   'text': '"Hey, first go look at the form. Look at it very\xa0\n'
           'carefully. Make sure you can tell what boxes are\xa0\xa0'},
  {'duration': 5.84,
   'start': 1093.2,
   'text': "checked. Make sure you're not missing anything\xa0\n"
           'here. Um, make a list for yourself of what you see\xa0\xa0'},
  {'duration': 5.12,
   'start': 1099.04,
   'text': 'in that. And then move on to the sketch. So after\xa0\n'
           "you've kind of confidently gotten information\xa0\xa0"},
  {'duration': 6.32,
   'start': 1104.16,
   'text': "out of the form and you can say what's factually\xa0\n"
           'true, then you can go on and think about what you\xa0\xa0'},
  {'duration': 5.92,
   'start': 1110.48,
   'text': 'can gain from that sketch. keeping in mind your\xa0\n'
           'understanding of the accident so far. So, whatever\xa0\xa0'},
  {'duration': 3.68,
   'start': 1116.4,
   'text': "you've learned from the form, you're trying to\xa0\n"
           "match that up with the sketch. And that's how\xa0\xa0"},
  {'duration': 12.24,
   'start': 1120.08,
   'text': "you're going to arrive um at your final uh at your\xa0\n"
           "final assessment of the form. And we'll run it."},
  {'duration': 4.56,
   'start': 1132.32,
   'text': 'And here you can see one behavior that\xa0\n'
           'this produced for Claude because I told\xa0\xa0'},
  {'duration': 4.8,
   'start': 1136.88,
   'text': "it to very carefully examine the form. It's\xa0\n"
           'showing me its work as it does that. So,\xa0\xa0'},
  {'duration': 5.28,
   'start': 1141.68,
   'text': "it's telling me each individual box. Is the box\xa0\n"
           'checked? Is it not checked? And so, this is one\xa0\xa0'},
  {'duration': 4.4,
   'start': 1146.96,
   'text': "thing you'll notice as you do prompt engineering.\xa0\n"
           'In our previous prompts, we were kind of letting\xa0\xa0'},
  {'duration': 4.96,
   'start': 1151.36,
   'text': 'claw decide how much it wanted to tell us about\xa0\n'
           "what it saw on the form here. Because I've told\xa0\xa0"},
  {'duration': 5.6,
   'start': 1156.32,
   'text': "it carefully examine each and every box, it's very\xa0\n"
           'carefully examining each and every box. And that\xa0\xa0'},
  {'duration': 4.56,
   'start': 1161.92,
   'text': "might not be what we want in the end. So, that's\xa0\n"
           "something we might change. Um, but it's also going\xa0\xa0"},
  {'duration': 5.92,
   'start': 1166.48,
   'text': 'to give me these other things that I asked for\xa0\n'
           'in XML tags. So, a nice analysis of the form, the\xa0\xa0'},
  {'duration': 5.04,
   'start': 1172.4,
   'text': "accident summary so far. It's going to give me a\xa0\n"
           "sketch analysis, and it's going to continue to say\xa0\xa0"},
  {'duration': 5.44,
   'start': 1177.44,
   'text': 'that vehicle B appears to be clearly at fault. In\xa0\n'
           "this in this example, it's pretty simple example\xa0\xa0"},
  {'duration': 5.76,
   'start': 1182.88,
   'text': 'with more complicated drawings, more uh less\xa0\n'
           'clarity in the forms. This kind of step-by-step\xa0\xa0'},
  {'duration': 5.44,
   'start': 1188.64,
   'text': 'thinking for Claude is really impactful in\xa0\n'
           'its ability to make a correct assessment here.\xa0\xa0'},
  {'duration': 4.56,
   'start': 1194.72,
   'text': "Uh, so I think we'll go back to the slides and\xa0\n"
           "Christian's going to talk about a last kind of\xa0\xa0"},
  {'duration': 5.76,
   'start': 1199.28,
   'text': 'piece that we might add to this um to really make\xa0\n'
           'it useful for a real world task. Indeed. Thank\xa0\xa0'},
  {'duration': 6.48,
   'start': 1205.04,
   'text': 'you so much. So, as Hannah mentioned, uh, we sort\xa0\n'
           'of set the stage in this prompt to make sure that\xa0\xa0'},
  {'duration': 5.36,
   'start': 1211.52,
   'text': 'really acting on our behalf in a right manner.\xa0\n'
           'Um, and a key step that we also add towards the\xa0\xa0'},
  {'duration': 4.72,
   'start': 1216.88,
   'text': "end of this prompt that I'm going to show you in a\xa0\n"
           'second is a simple sort of guidelines or reminder\xa0\xa0'},
  {'duration': 4.4,
   'start': 1221.6,
   'text': 'part as well. just strengthening and reinforcing\xa0\n'
           'exactly what we want to get out of it. And one\xa0\xa0'},
  {'duration': 4.56,
   'start': 1226.0,
   'text': 'important piece is actually output formatting.\xa0\n'
           "You can imagine if you're a data engineer working\xa0\xa0"},
  {'duration': 4.8,
   'start': 1230.56,
   'text': 'on this LM application, all the sort of fancy\xa0\n'
           'preamble is great, but at the end of the day,\xa0\xa0'},
  {'duration': 4.4,
   'start': 1235.36,
   'text': 'you want your piece of information to to\xa0\n'
           "be stored in, let's say, your SQL database,\xa0\xa0"},
  {'duration': 5.04,
   'start': 1239.76,
   'text': 'wherever you want to store that data. And the rest\xa0\n'
           'of it that is necessary for cloud to sort of give\xa0\xa0'},
  {'duration': 4.72,
   'start': 1244.8,
   'text': "its verdict isn't really that necessary for your\xa0\n"
           'application. You want the nitty-gritty information\xa0\xa0'},
  {'duration': 5.36,
   'start': 1249.52,
   'text': 'for your application. So if we transition back to\xa0\n'
           "the console, you'll see here that we just added\xa0\xa0"},
  {'duration': 6.8,
   'start': 1254.88,
   'text': 'a simple importance guidelines part. And again,\xa0\n'
           'this is just reinforcing the sort of mechanical\xa0\xa0'},
  {'duration': 4.32,
   'start': 1261.68,
   'text': 'behavior that we want out of cloud here. Want\xa0\n'
           'to make sure that the summary is clear, concise,\xa0\xa0'},
  {'duration': 5.36,
   'start': 1266.0,
   'text': 'and accurate. Want to make sure that nothing\xa0\n'
           "is sort of impeding in in in Claw's assessment\xa0\xa0"},
  {'duration': 4.32,
   'start': 1271.36,
   'text': "apart from the data it's analyzing. And then\xa0\n"
           'finally, when it comes to output formatting,\xa0\xa0'},
  {'duration': 4.8,
   'start': 1275.68,
   'text': "in my case here, I'm just going to ask Claude\xa0\n"
           "to wrap its final verdict. All other stuff I'm\xa0\xa0"},
  {'duration': 3.6,
   'start': 1280.48,
   'text': 'actually going to ignore for my application and\xa0\n'
           "just look at what it's actually assessing. And\xa0\xa0"},
  {'duration': 5.2,
   'start': 1284.08,
   'text': 'that is I can I can use this if I want to build\xa0\n'
           'some sort of analytics tool afterwards as well.\xa0\xa0'},
  {'duration': 5.92,
   'start': 1289.28,
   'text': 'Or if I just want a clearcut um uh determination,\xa0\n'
           'this is a way I can do so. So if I just run this\xa0\xa0'},
  {'duration': 4.16,
   'start': 1295.2,
   'text': "here, you'll see it's going through the same sort\xa0\n"
           "of process that we've seen before. In this case,\xa0\xa0"},
  {'duration': 4.16,
   'start': 1299.36,
   'text': "it's much more succinct because we've asked\xa0\n"
           'to be to summarize its findings in a much\xa0\xa0'},
  {'duration': 4.96,
   'start': 1303.52,
   'text': 'more straightforward manner. And then finally\xa0\n'
           "towards the end you'll see that it'll wrap my\xa0\xa0"},
  {'duration': 5.6,
   'start': 1308.48,
   'text': 'output in these final verdict XML tags. So you\xa0\n'
           "can see that during this demo we've gone from\xa0\xa0"},
  {'duration': 7.6,
   'start': 1314.08,
   'text': 'a skiing accident to sort of unconfident insecure\xa0\n'
           'outputs from perhaps a car accident in the second\xa0\xa0'},
  {'duration': 6.64,
   'start': 1321.68,
   'text': 'version to now a much more strictly formatted\xa0\n'
           'confident output that we can actually build an\xa0\xa0'},
  {'duration': 7.36,
   'start': 1328.32,
   'text': 'application around and actually help you know a\xa0\n'
           'real world um car insurance company for example.\xa0\xa0'},
  {'duration': 8.64,
   'start': 1335.68,
   'text': 'U finally if we transition back to the um slides\xa0\n'
           "another key way of shaping CL's output is actually\xa0\xa0"},
  {'duration': 5.92,
   'start': 1344.32,
   'text': "putting words in CL's mouth or as we call it\xa0\n"
           'pre-filled responses. You could imagine that\xa0\xa0'},
  {'duration': 6.08,
   'start': 1350.24,
   'text': 'parsing XML tags is nice and all but maybe you\xa0\n'
           'want a structured JSON output to make sure that\xa0\xa0'},
  {'duration': 5.76,
   'start': 1356.32,
   'text': "uh it's JSON serializable and you can use this\xa0\n"
           'in a subse subsequent call for example. Um this\xa0\xa0'},
  {'duration': 6.08,
   'start': 1362.08,
   'text': 'is quite simple to do. You could just add that um\xa0\n'
           'claude needs to begin its output with a certain\xa0\xa0'},
  {'duration': 5.2,
   'start': 1368.16,
   'text': 'format. This could be for example a uh open\xa0\n'
           'square bracket squarely bracket for example\xa0\xa0'},
  {'duration': 4.72,
   'start': 1373.36,
   'text': 'or even in this case that we see in front of us\xa0\n'
           'this would be an XML tag for itinerary. In our\xa0\xa0'},
  {'duration': 6.0,
   'start': 1378.08,
   'text': 'case it could also be that final verdict XML tag.\xa0\n'
           'Um, and this is just a great way of again shaping\xa0\xa0'},
  {'duration': 5.6,
   'start': 1384.08,
   'text': 'how Claude is supposed to respond. Um, without all\xa0\n'
           "the preamble if you don't want that, even though\xa0\xa0"},
  {'duration': 4.32,
   'start': 1389.68,
   'text': 'that is also key in shaping his output to make\xa0\n'
           'sure that Claude is reasoning through the steps\xa0\xa0'},
  {'duration': 4.72,
   'start': 1394.0,
   'text': 'that we wanted. So in our case here, we would just\xa0\n'
           'wrap it in the final verdict and then parse it\xa0\xa0'},
  {'duration': 6.24,
   'start': 1398.72,
   'text': 'afterwards. But you can use prefill as well. Now\xa0\n'
           'finally one step that I would like to highlight\xa0\xa0'},
  {'duration': 6.72,
   'start': 1404.96,
   'text': 'here as well is that both cloud 3.7 and especially\xa0\n'
           'cloud 4 of course is a sort of hybrid reasoning\xa0\xa0'},
  {'duration': 4.56,
   'start': 1411.68,
   'text': "model meaning that there's extended thinking at\xa0\n"
           'your disposal. Um and this is something we want\xa0\xa0'},
  {'duration': 5.68,
   'start': 1416.24,
   'text': 'to highlight because you can use extended thinking\xa0\n'
           'as a crutch for your prompt engineering. Basically\xa0\xa0'},
  {'duration': 3.68,
   'start': 1421.92,
   'text': 'you can enable this to make sure that Claude\xa0\n'
           'actually has time to think. It adds his thinking\xa0\xa0'},
  {'duration': 4.48,
   'start': 1425.6,
   'text': 'tags and the scratch pad. Um and the beauty of\xa0\n'
           'that is you can actually analyze that transcript\xa0\xa0'},
  {'duration': 4.96,
   'start': 1430.08,
   'text': 'to understand how claude is going about that data.\xa0\n'
           'So as we mentioned we have these check boxes where\xa0\xa0'},
  {'duration': 5.12,
   'start': 1435.04,
   'text': 'it goes through step by step of the scenario\xa0\n'
           'that transpired for the accident. And in many\xa0\xa0'},
  {'duration': 4.48,
   'start': 1440.16,
   'text': 'ways there you can actually try to help claude in\xa0\n'
           "building this into the system prompt itself. It's\xa0\xa0"},
  {'duration': 5.36,
   'start': 1444.64,
   'text': "not only more token efficient but it's a good way\xa0\n"
           'of understanding how these intelligent models that\xa0\xa0'},
  {'duration': 4.32,
   'start': 1450.0,
   'text': "don't have our intuition actually go about the\xa0\n"
           'data that we provide them. And because of that,\xa0\xa0'},
  {'duration': 5.36,
   'start': 1454.32,
   'text': "it's quite key in actually trying to break down\xa0\n"
           'how your system prompt can get a lot better. Um,\xa0\xa0'},
  {'duration': 4.88,
   'start': 1459.68,
   'text': "and with that said, I think uh I'd like to thank\xa0\n"
           "all you for coming today. We'll be around as well.\xa0\xa0"},
  {'duration': 4.48,
   'start': 1464.56,
   'text': 'So if you have any questions on prompting, please\xa0\n'
           "uh please go ahead. I know there's a prompting.\xa0\xa0"},
  {'duration': 4.96,
   'start': 1469.04,
   'text': 'You want to learn more about prompting in an hour.\xa0\n'
           'We have prompting for agents and right now we\xa0\xa0'},
  {'duration': 5.2,
   'start': 1474.0,
   'text': 'have an amazing demo of Claude plays Pokemon. So\xa0\n'
           "don't go anywhere for that. And as Christian said,\xa0\xa0"},
  {'duration': 2.96,
   'start': 1479.2,
   'text': "we'll be around all day. So, I know we\xa0\n"
           "didn't have time for Q&A in this session,\xa0\xa0"},
  {'duration': 7.12,
   'start': 1482.16,
   'text': 'but uh please come find us if you want to chat.\xa0\n'
           'And thank you guys for coming. Thank you so much.'}]]
