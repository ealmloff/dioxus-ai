pub const PROMPT: &str = r#"You generate snippets of JSX. You will only use tailwindcss for styling. You may include comments to explain the HTML with the `<!-- comment -->` syntax.

The UI should be:
- Responsive
- Mobile-first
- Accessible
- SEO-friendly
- Well documented

You should avoid:
- Using JavaScript

You may never use:
- Any libraries
- SVG
- Inline images

You always follow this response format:
1) What should the UI look like?
2) What are the individual components that make up the UI? Name each component with an upper camel case identifier and specify if the component is standalone or takes children.
3) What does the HTML for the UI look like? Insert `<ComponentName/>` in the HTML where you want the component to be inserted. This **must** be valid HTML. May not contain `{Children}` placeholder.
4) What is the HTML for each component? Components may render child elements with the special `{children}` placeholder.

For any information you don't know in the HTML. Use `{lower_camel_case_identifier}` in the HTML instead of the information. The information must be a string or number only.
For example, if you don't know how many downloads a library has, you might put <p>{download_count} downloads</p> in the HTML."#;

pub const UI_COMPONENTS: &[&str] = &[
    "Thumbnail generator for video previews",
    "Video player with playback controls",
    "Search bar for finding videos",
    "Video upload tool with encoding options",
    "User profile page with video history",
    "Comment section for video discussions",
    "Video category browser with filtering",
    "Recommended videos carousel",
    "Video sharing buttons for social media",
    "Video analytics dashboard for creators",
    "Subscription management for channel updates",
    "Live streaming tools with real-time chat",
    "Video metadata editor for title and tags",
    "Content moderation tools for admin teams",
    "Post Feed List: Displays a list of user posts",
    "Post Card: A single post with user info and actions",
    "Post Text Editor: Input field for writing a new post",
    "Comment Thread: Display comments and replies",
    "Comment Input: Input field for writing a new comment",
    "User Profile Card: Compact user info and profile link",
    "NavBar: Main navigation menu for the site",
    "Search Bar: Input field for searching the site",
    "Trending Topics: List of popular hashtags and topics",
    "Friends List: List of friends with profile links",
    "Notification Badge: Displays new notification count",
    "Loading Spinner: Animated loading indicator",
    "Error Message: Displays error message with details",
    "Login Form: Form for logging in to the site",
    "Share Button: Button for sharing content",
    "Like Button: Button for liking a post",
    "Comment Button: Button for commenting on a post",
    "Avatar Upload: Input field for uploading a profile picture",
    "Cover Photo Upload: Input field for uploading a cover photo",
    "Alert Popup: Popup with important alert message",
    "Header bar with logo and navigation",
    "Search bar for finding specific content",
    "Post list with title, score, and comments",
    "Single post with text, image, and comments",
    "Comment thread with replies and nested comments",
    "User profile with karma, post history, and info",
    "Sidebar with popular posts and trending topics",
    "Footer with links to policies, terms, and help",
    "Breadcrumb navigation for category and subreddits",
    "Button for submitting a new post or comment",
    "Sorting options for posts by top, new, and hot",
    "Warning or error message for invalid input",
    "Modal dialog for creating a new post or comment",
    "Paging controls for navigating through posts",
    "Avatar or thumbnail representing a user",
    "Category or subreddit selection dropdown",
    "Upvote and downvote buttons for rating content",
    "Tweet input field for user input",
    "Character counter for tweet length",
    "Send button to post new tweet",
    "Avatar display for user profile picture",
    "Username display for other users",
    "Follow button to subscribe to users",
    "Unfollow button to unsubscribe from users",
    "Tweet display for individual posts",
    "Timestamp display for post timing",
    "Like button to show tweet approval",
    "Retweet button to share tweets",
    "Reply button to respond to tweets",
    "Comment display for conversation threads",
    "Mention display for tagged users",
    "Hashtag display for topic tracking",
    "Search bar for finding specific content",
    "Navigation menu for app sections",
    "Notification bell for new updates",
    "User profile summary and bio",
    "Tweet edit button for post revisions",
    "Tweet delete button for post removal",
    "Direct message button for private chats",
    "Trending topics display for popular discussions",
    "Who to follow suggestions for users",
    "Tweet analytics for engagement metrics",
    "Navigation menu with mailbox folders and labels",
    "Search bar with autocomplete suggestions",
    "Compose email button with dropdown menu",
    "Email list view with sender and subject",
    "Individual email thread with reply buttons",
    "Email composition window with formatting options",
    "Attachment upload button with file browser",
    "Contact list with avatar and online status",
    "Settings icon with dropdown menu options",
    "Notification bell with unread count badge",
    "Profile picture with dropdown menu options",
    "Logout button with confirmation prompt",
    "Folder and label management interface",
    "Email filtering and sorting options",
    "Message body text editor with formatting tools",
    "Recipient auto-complete dropdown list",
    "Send and save draft buttons",
    "Email signature editor with formatting options",
    "Code Editor with Syntax Highlighting",
    "Project Timeline Visualization Tool",
    "Repository File Navigation System",
    "Collaborator Permission Management System",
    "Real-time Comment and Discussion Forum",
    "Code Review and Approval Workflow",
    "Personalized Dashboard with Project Insights",
    "Drag-and-Drop FileUploader Component",
    "Customizable Task and Issue Tracker",
    "Integrated Wiki and Documentation Editor",
    "User Profile and Reputation System",
    "Repository Search and Filtering Tool",
    "Dependancy Graph Visualization Component",
    "Commit History and Blame Viewer",
    "Pull Request Management System",
    "Hero Header with Background Image",
    "Product Showcase with Animated Cards",
    "Call-to-Action (CTA) Button Section",
    "Customer Testimonial with Profile Picture",
    "Split-Screen Comparison Section",
    "Dynamic Product Feature List",
    "High-Quality Image Carousel",
    " Animated Counter Statistics Section",
    "Simple and Minimalistic Footer Section",
    "Video Background with Overlay Text",
    "Interactive Product Demo Simulation",
    "Featured Story with Large Header Text",
    "Benefits-Focused Bullet Point List",
    "Grid-Based Product Gallery Section",
    "Scroll-Activated Animation Effect",
    "Hero Section: Showcase your product with a bold statement",
    "Features Grid: Highlight key benefits in a grid layout",
    "Testimonial Slider: Showcase social proof from happy customers",
    "Call-to-Action (CTA) Button: Encourage visitors to take action",
    "Product Image Gallery: Showcase your product from multiple angles",
    "Benefits Section: Highlight the benefits of your product",
    "Statistics Section: Showcase impressive metrics and data",
    "Pricing Table: Display pricing options and plans",
    "Customer Logos: Showcase trusted partners and clients",
    "Footer Section: Provide additional resources and links",
    "Hero Video: Engage visitors with a captivating video",
    "Screenshot Carousel: Showcase your product in action",
    "Trust Badges: Establish credibility with security badges",
    "Features List: Break down complex features into bite-sized chunks",
    "Header: Site title and navigation links",
    "Hero Image: High-resolution background image",
    "Featured Posts: Showcase of top articles",
    "Sidebar: Important links and widgets",
    "Footer: Copyright and contact information",
    "Post List: Teasers for latest articles",
    "Article Title: Headline of individual post",
    "Post Content: Main text and media of article",
    "Comment Section: User discussions and feedback",
    "Author Bio: Information about article's author",
    "Social Sharing: Buttons for sharing content",
    "Related Posts: Suggestions for further reading",
    "Pagination: Navigation for multiple pages",
    "Search Bar: Input field for searching content",
    "Popular Tags: Cloud of frequently used keywords",
    "Category List: Links to categorized content",
    "Archives: List of past articles by date",
    "Call-to-Action: Prominent button or link",
    "Header with logo and navigation menu",
    "Hero section with background image and tagline",
    "About me section with bio and profile picture",
    "Projects showcase with thumbnails and descriptions",
    "Blog section with latest posts and archives",
    "Contact form with name, email, and message fields",
    "Social media links with icons and tooltips",
    "Footer with copyright information and disclaimer",
    "Responsive navigation menu for mobile devices",
    "Call-to-action button with animation effects",
    "Testimonials section with quotes and author names",
    "Skills section with icons and proficiency levels",
    "Experience timeline with job titles and dates",
    " Education section with degrees and institutions",
    "Personal interests and hobbies section with icons",
    "Hero section with animated typography",
    "Skill meter with percentage counters",
    "Service section with icons and brief descriptions",
    "Portfolio filter with categories and tags",
    "Modal window with project details and images",
    "Resume download button with print option",
    "Personal quote or mantra with fade-in animation",
    "Scroll-to-top button with smooth animation",
    "News or updates section with date and title",
    "Award or achievement section with icons and dates",
    "Certifications or licenses section with issuers",
    "Team or colleagues section with photos and bios",
    "Client or testimonial logos with carousel",
    "Social media feed with latest posts and images",
    "User profile popup with avatar and username",
    "Channel list with collapsible categories and unread counts",
    "Message input field with send button and emoji picker",
    "Threaded conversation view with reply and reaction buttons",
    "Server icon and name display with dropdown menu",
    "Rich media embed with video playback and playback controls",
    "Mentions dropdown with user and role mentions",
    "Reaction picker with popular and custom emoji reactions",
    "Direct message conversation list with online status",
    "Verified checkmark and badge for official channels",
    "Loading spinner with animation and progress circle",
    "Error message popup with retry and cancel buttons",
    "User settings panel with notification and account options",
    "Channel topic and description with edit button",
    "Pinned messages with timestamp and author display",
    "Upload file input field with file type and size limits",
    "Server role management with permission editting",
    "Emoji picker with search and category filtering",
    "Dashboard: Visualize key metrics and KPIs",
    "Lead Form: Capture prospect information",
    "Contact List: Manage customer relationships",
    "Task Calendar: Schedule and assign tasks",
    "Opportunity Tracker: Monitor sales pipelines",
    "Custom Field Builder: Create tailored data fields",
    "Role-Based Access: Control user permissions",
    "Revenue Forecast: Predict future sales",
    "Sales Pipeline: Visualize deal stages",
    "Customer Profile: View customer details",
    "Reporting Dashboard: Generate data insights",
    "Integrations Hub: Connect third-party apps",
    "User Management: Manage user accounts",
    "Data Importer: Upload bulk data",
    "Customizable Themes: Brand your interface",
    "Notifications Panel: Receive system updates",
    "Reporting Wizard: Simplify report creation",
    "Chart Builder: Visualize data trends",
    "Security Center: Monitor system security",
    "API Explorer: Discover APIs and endpoints",
    "Search bar for finding products",
    "Product image carousel with zoom",
    "Product title and rating display",
    "Price and availability information",
    "Add to cart and wishlist buttons",
    "Product description and details section",
    "Customer review and rating summaries",
    "Suggested products carousel",
    "Breadcrumbs for navigation",
    "Product filtering and sorting controls",
    "Faceted search results display",
    "Responsive side navigation menu",
    "Simple product listing with thumbnails",
    "Detailed product information table",
    "Product variant selector (e.g. size, color)",
    "Secure payment information input",
    "Order summary and subtotal display",
    "Shipping and delivery options",
    "Product Cards/Grids",
    "Shopping Cart Icon",
    "Search Bar",
    "Category Navigation",
    "Product Images/Zoom",
    "Ratings and Reviews",
    "Price and Discount Badges",
    "Call-to-Action Buttons (CTAs)",
    "Slider/Carousel for Promotions",
    "Responsive Filtering and Sorting",
    "Hero Image/Header",
    "Article Thumbnail Images",
    "Headline and Summary Text",
    "Author and Date Metadata",
    "Social Sharing Buttons",
    "Tags and Categories",
    "Comment Section",
    "Related Articles/Recommended Content",
    "Author Bio/Profile",
    "Responsive Article Layout",
    "Profile Picture and Header Image",
    "Post/Tweet Box",
    "Feed/List View of Posts",
    "Like/Comment/Share Buttons",
    "User Profile Cards",
    "Post/Tweet Embeds",
    "Hashtag and @Mention Links",
    "Profile Navigation (e.g. tabs, sidebar)",
    "Notification Alerts/Badges",
    "Responsive Post Formatting",
    "Service/Gig Cards",
    "Provider Profiles",
    "Rating and Review Systems",
    "Job/Project Listings",
    "Bidding/Auction Functions",
    "Payment and Invoicing Tools",
    "Time Tracking and Scheduling",
    "Messaging and Chat Systems",
    "Project/Task Management Tools",
    "Responsive Listing Grids",
    "Course Cards/Grids",
    "Instructor/Author Profiles",
    "Course Progress Bars",
    "Lesson/List View of Content",
    "Video/Audio Players",
    "Quiz and Assessment Tools",
    "Discussion Forums and Comments",
    "Certificates and Badges",
    "Course Recommendations",
    "Responsive Video Embeds",
    "Thread/List View of Topics",
    "Post Reply and Editing Tools",
    "User Profile and Reputation Systems",
    "Threaded Commenting and Nesting",
    "Tag and Category Navigation",
    "Search Bar and Filters",
    "User Ranks and Badges",
    "Thread Favorites and Watching",
    "Post Editing and Formatting Tools",
    "Responsive Table Layout",
    "Project Cards/Grids",
    "Image Lightboxes and Zoom",
    "Designer/Studio Profiles",
    "Case Study and Description Text",
    "Tag and Category Navigation",
    "Search Bar and Filters",
    "Related Project Suggestions",
    "User Comments and Feedback",
    "Project Favorites and Bookmarking",
    "Responsive Image Galleries",
    "Forum Categories and Subforums",
    "Thread/List View of Topics",
    "Post Reply and Editing Tools",
    "User Profile and Reputation Systems",
    "Threaded Commenting and Nesting",
    "Tag and Category Navigation",
    "Search Bar and Filters",
    "User Ranks and Badges",
    "Thread Favorites and Watching",
    "Responsive Table Layout",
    "Destination and Hotel Cards",
    "Map View of Locations",
    "Availability and Pricing Calendars",
    "Room Type and Rate Selection",
    "Booking and Payment Forms",
    "Traveler Reviews and Ratings",
    "Hotel and Attraction Photos",
    "Trip Itinerary and Booking Summary",
    "Currency and Language Switchers",
    "Responsive Maps and Directions",
    "Game Tiles and Grids",
    "Game Trailer and Video Embeds",
    "User Profiles and Achievements",
    "Game Reviews and Ratings",
    "Recommendations and 'You Might Like'",
    "Community Forum and Chat Tools",
    "Leaderboards and Rankings",
    "Game Information and Stats",
    "Download and Installation Tools",
    "Responsive Button and Icon Design",
    "Accordion/Toggle Components",
    "Alert/Banner Notifications",
    "Audio Players",
    "Background Images/Patterns",
    "Breadcrumb Navigation",
    "Button Groups",
    "Call-to-Action Buttons",
    "Card/Tile Grids",
    "Carousel/Slider Components",
    "Chart/Graph Visualizations",
    "Checkout/Order Summary",
    "Comments/Review Section",
    "Comparison Tables",
    "Contact/Feedback Forms",
    "Content Accordions",
    "Cookie Banners",
    "Countdown Timers",
    "Coupon/Discount Codes",
    "Custom Scrollbars",
    "Dashboard/Analytics Charts",
    "Date Pickers",
    "Dialog/Prompt Boxes",
    "Dividers/Section Separators",
    "Drag-and-Drop Interfaces",
    "Editable Tables",
    "Email Newsletter Signups",
    "Error/Success Messages",
    "FAQ/Accordion Sections",
    "Filters and Sorting",
    "Floating Action Buttons",
    "Footer/Social Links",
    "Form Validation Feedback",
    "Gallery/Image Lightboxes",
    "Geo-location Map Markers",
    "GloballyUniqueIdentifiableElements",
    "Google Maps Embeds",
    "Header/Footer Navigation",
    "Hero/Banner Images",
    "Hover Effects/Animations",
    "Icon Fonts/SVG Sprites",
    "Image Carousels",
    "Infinite Scroll/Pagination",
    "Inline Editing/CRUD",
    "Intercept Page/Modal Dialogs",
    "JSON Data Visualization",
    "Keyboard-Navigable interfaces",
    "Loading Animations/Spinners",
    "Login/Registration Forms",
    "Modal/Dialog Windows",
    "Navbar/Mega Menu",
    "News Ticker/Marquee",
    "Nested Comment Threads",
    "Off-Canvas/Flyout Menus",
    "On-hover Tooltips",
    "Pagination/Infinite Scroll",
    "Password Strength Meters",
    "Pie Charts/Doughnut Charts",
    "Placeholder/Loading Text",
    "Playlist/Audio Controls",
    "Poll/Voting Systems",
    "Price/Discount Tables",
    "Progress Bars/Steps",
    "Pull Quotes/Testimonials",
    "Radio/Checkbox Inputs",
    "Rating Systems/Reviews",
    "Real-time Updates/Feeds",
    "Responsive Image Breakpoints",
    "Revealing/Hover Effects",
    "Rich Text Editors",
    "Right-to-Left (RTL) Support",
    "Roadmap/Gantt Charts",
    "Search Bar/Autocomplete",
    "Semantic HTML5 Markup",
    "SEO Meta Tags/Optimization",
    "Shader/Gradient backgrounds",
    "Sharrre and Social Sharing",
    "Sidebars/Dockable Panels",
    "Skeleton Screens/Loading",
    "Sliders/Ranges/Scalars",
    "Social Media Embedded Posts",
    "Sortable Tables/Grids",
    "Spinner/Loading Animations",
    "State/Province/Country Selects",
    "Sticky/Fixed Navigation",
    "Stock Chart/Candlestick",
    "Stencil Portals/Masking",
    "Sticky Header/Footer",
    "Switches/Toggles/Buttons",
    "Tabbed Interfaces/Panels",
    "Tag Clouds/Folksonomy",
    "Targets/Anchor Links",
    "Telephone/Phone Number Inputs",
    "Text Inputs/Auto-complete",
    "Time Pickers/Clock Icons",
    "Toast/Notification Messages",
    "Toggle Buttons/Radio Groups",
    "Toolbar/Dropdown Menus",
    "Top/Bottom Navigation",
    "Tooltip/Hovercard Previews",
    "Tree View/ Folder Navigation",
    "Two-Factor Auth/QR Codes",
    "Typography/Font Styles",
    "Unobtrusive JavaScript",
    "Upload/Progress Bars",
    "User Avatars/Profile Pics",
    "Vertical Tabs/Navigation",
    "Video Players/Embeds",
    "Virtual Scrolling",
    "Voice Command/Audio Feedback",
    "Webcam/Camera Interfaces",
    "WebSocket/Real-time Updates",
    "WYSIWYG/Rich Text Editors",
    "X-Axis/Y-Axis Charts",
    "Year/Month Picker Calendars",
    "YouTube/Vimeo Embeds",
    "Zoom/Magnify Effects",
    "Z-Index/Stacking Contexts",
    "Header Navigation Bar",
    "Footer Copyright Text",
    "Hero Background Image",
    "Call to Action",
    "Featured Product Card",
    "Blog Post Teaser",
    "Social Media Icons",
    "Search Input Field",
    "Dropdown Menu List",
    "Breadcrumb Navigation",
    "Page Title Header",
    "Featured Image Slider",
    "Testimonial Quote Box",
    "Button Group Container",
    "Alert Notification Box",
    "Footer Contact Info",
    "Header Logo Image",
    "Product Rating Stars",
    "Pagination Links List",
    "Featured Video Player",
    "Blog Post Archive",
    "Sidebar Widget Area",
    "Callout Box Container",
    "Header Search Icon",
    "Footer Sitemap List",
    "Hero Overlay Text",
    "Featured Image Grid",
    "Social Sharing Buttons",
    "Page Content Header",
    "Dropdown Menu Arrow",
    "Breadcrumb Separator",
    "Product Price Tag",
    "Testimonial Avatar Image",
    "Alert Close Button",
    "Footer Newsletter Form",
    "Header Navigation Links",
    "Featured Product Badge",
    "Blog Post Comment Form",
    "Callout Box Title",
    "Hero Background Video",
    "Pagination Previous Link",
    "Social Media Feed",
    "Header Language Switcher",
    "Footer Terms Link",
    "Featured Image Carousel",
    "Product Description Text",
    "Testimonial Quote Text",
    "Alert Notification Icon",
    "Footer Copyright Year",
    "Header User Profile",
    "Blog Post Author Name",
    "Callout Box Button",
    "Hero Overlay Gradient",
    "Featured Product Image",
    "Social Sharing Link",
    "Page Content Text",
    "Dropdown Menu Item",
    "Breadcrumb Home Link",
    "Product Rating Count",
    "Footer Sitemap Title",
    "Header Search Input",
    "Featured Video Thumbnail",
    "Blog Post Category",
    "Sidebar Widget Title",
    "Callout Box Text",
    "Hero Background Pattern",
    "Featured Image Lightbox",
    "Social Media Profile",
    "Page Content Image",
    "Dropdown Menu Title",
    "Breadcrumb Separator Icon",
    "Product Price Currency",
    "Testimonial Author Name",
    "Alert Close Icon",
    "Footer Newsletter Text",
    "Header Navigation Icon",
    "Featured Product Title",
    "Blog Post Date Published",
    "Callout Box Link",
    "Hero Overlay Text Color",
    "Featured Image Zoom",
    "Social Sharing Count",
    "Page Content Header Tag",
    "Dropdown Menu Chevron",
    "Breadcrumb Current Page",
    "Product Description List",
    "Testimonial Quote Mark",
    "Alert Notification Text",
    "Footer Copyright Symbol",
    "Header User Avatar",
    "Blog Post Tags List",
    "Callout Box Background",
    "Hero Background Image Size",
    "Featured Product Badge Icon",
    "Social Media Follow Button",
    "Page Content Paragraph",
    "Dropdown Menu Divider",
    "Breadcrumb Home Icon",
    "Product Rating Average",
    "Footer Sitemap Icon",
    "Header Search Button",
    "Featured Video Poster",
    "Blog Post Author Image",
    "Sidebar Widget Content",
    "Callout Box Padding",
    "Hero Overlay Opacity",
    "Featured Image Caption",
    "Social Sharing Button",
    "Page Content List Item",
    "Dropdown Menu Item Icon",
    "Breadcrumb Separator Line",
    "Product Price Discount",
    "Testimonial Quote Image",
    "Alert Notification Icon Color",
    "Footer Newsletter Label",
    "Header Navigation Menu",
    "Featured Product Image Size",
    "Blog Post Comment Count",
    "Callout Box Border Radius",
    "Hero Background Color",
    "Featured Image Gallery",
    "Social Media Profile Picture",
    "Page Content Table Header",
    "Dropdown Menu Submenu",
    "Breadcrumb Current Text",
    "Product Description Table",
    "Testimonial Author Image",
    "Alert Close Button Icon",
    "Footer Copyright Information",
    "Header User Dropdown",
    "Blog Post Tags Cloud",
    "Callout Box Shadow",
    "Hero Overlay Gradient",
    "Featured Image Lightbox Size",
    "Social Sharing Network",
    "Page Content Ordered List",
    "Dropdown Menu Item Text",
    "Breadcrumb Home Text",
    "Product Rating Distribution",
    "Testimonial Quote Style",
    "Alert Notification Style",
    "Footer Newsletter Input",
    "Header Navigation Toggle",
    "Featured Product Variations",
    "Blog Post Author Bio",
    "Callout Box Background Image",
    "Hero Overlay Pattern",
    "Featured Image Carousel Navigation",
    "Social Media Share Button",
    "Page Content Unordered List",
    "Dropdown Menu Item Icon Color",
    "Breadcrumb Separator Style",
    "Product Price Original",
    "Testimonial Author Title",
    "Alert Close Button Style",
    "Footer Newsletter Submit",
    "Header Search Suggestion",
    "Featured Product Reviews",
    "Blog Post Comment Form Label",
    "Callout Box Text Color",
    "Hero Overlay Opacity Level",
    "Featured Image Zoom Level",
    "Social Media Follow Count",
    "Page Content Header Image",
    "Dropdown Menu Item Style",
    "Breadcrumb Current Page Text",
    "Product Description Bullet",
    "Testimonial Quote Font",
    "Alert Notification Sound",
    "Footer Newsletter Success",
    "Header User Profile Picture",
    "Blog Post Category List",
    "Callout Box Border Style",
    "Hero Overlay Background",
    "Hero Section with Image",
    "Call to Action Button",
    "Featured Product Card",
    "Navigation Menu Bar",
    "Footer Social Links",
    "Testimonial Quote Box",
    "Blog Post Preview Card",
    "Featured Image Carousel",
    "Product Feature List",
    "Company Logo Icon",
    "Search Input Field",
    "Dropdown Menu List",
    "Featured Video Player",
    "Customer Review Stars",
    "Pricing Plan Table",
    "Hero Section with Text",
    "Social Media Feed",
    "Featured Image Gallery",
    "Call to Action Link",
    "Footer Copyright Text",
    "Product Description Text",
    "Navigation Breadcrumb",
    "Featured Product Image",
    "Testimonial Author Photo",
    "Blog Post Category List",
    "Featured Video Thumbnail",
    "Company Address Info",
    "Search Result Item",
    "Product Feature Icon",
    "Customer Review Text",
    "Hero Section with Video",
    "Navigation Menu Item",
    "Featured Product Title",
    "Social Media Icon Link",
    "Footer Newsletter Form",
    "Product Description List",
    "Call to Action Button",
    "Featured Image Slider",
    "Testimonial Quote Text",
    "Blog Post Author Info",
    "Navigation Dropdown Icon",
    "Featured Product Price",
    "Company Phone Number",
    "Search Input Placeholder",
    "Product Feature List Item",
    "Footer Social Media Link",
    "Hero Section with Form",
    "Navigation Menu Toggle",
    "Featured Video Poster",
    "Customer Review Rating",
    "Product Description Text",
    "Blog Post Category Link",
    "Featured Image Lightbox",
    "Call to Action Link Text",
    "Footer Copyright Year",
    "Navigation Menu Item Icon",
    "Featured Product Image Alt",
    "Testimonial Author Name",
    "Social Media Feed Item",
    "Product Feature Icon List",
    "Hero Section with Text",
    "Navigation Breadcrumb Link",
    "Featured Video Controls",
    "Company Email Address",
    "Search Result Item Link",
    "Product Description List Item",
    "Call to Action Button Text",
    "Featured Image Carousel Nav",
    "Testimonial Quote Author",
    "Blog Post Category Name",
    "Navigation Menu Submenu",
    "Featured Product Price Tag",
    "Social Media Icon Set",
    "Footer Newsletter Input",
    "Product Feature Description",
    "Hero Section with Image",
    "Navigation Menu Item Text",
    "Featured Video Play Button",
    "Customer Review Profile",
    "Company Fax Number",
    "Search Input Clear Button",
    "Product Feature List Title",
    "Call to Action Link URL",
    "Footer Social Media Icon",
    "Hero Section with Form",
    "Navigation Menu Toggle Icon",
    "Featured Image Zoom Button",
    "Testimonial Quote Mark",
    "Blog Post Author Profile",
    "Social Media Feed Header",
    "Product Description Textarea",
    "Navigation Breadcrumb Trail",
    "Featured Video Poster Image",
    "Company Address Map",
    "Search Result Item Title",
    "Product Feature Icon Size",
    "Call to Action Button Color",
    "Featured Image Lightbox Close",
    "Testimonial Author Profile",
    "Footer Copyright Info",
    "Hero Section with Text",
    "Navigation Menu Submenu",
    "Featured Video Play Icon",
    "Social Media Feed Footer",
    "Product Feature List Item",
    "Call to Action Link Text",
    "Footer Newsletter Submit",
    "Navigation Menu Item Link",
    "Featured Image Carousel Nav",
    "Testimonial Quote Text",
    "Blog Post Category List",
    "Company Phone Number",
    "Search Input Placeholder",
    "Product Feature Description",
    "Hero Section with Image",
    "Navigation Menu Toggle Text",
    "Featured Video Controls Bar",
    "Customer Review Profile",
    "Social Media Icon Set",
    "Footer Newsletter Input",
    "Product Feature Icon List",
    "Call to Action Button Text",
    "Featured Image Lightbox Image",
    "Testimonial Author Name",
    "Blog Post Author Info",
    "Navigation Menu Submenu",
    "Featured Video Poster Image",
    "Company Email Address",
    "Search Result Item Link",
    "Product Feature List Title",
    "Call to Action Link URL",
    "Footer Social Media Icon",
    "Hero Section with Form",
    "Navigation Menu Toggle Icon",
    "Featured Image Zoom Button",
    "Testimonial Quote Mark",
    "Blog Post Category Name",
    "Social Media Feed Header",
    "Product Description Textarea",
    "Navigation Breadcrumb Trail",
    "Featured Video Play Button",
    "Company Address Map",
    "Search Result Item Title",
    "Product Feature Icon Size",
    "Call to Action Button Color",
    "Featured Image Lightbox Close",
    "Testimonial Author Profile",
    "Footer Copyright Info",
    "Hero Section with Text",
    "Navigation Menu Submenu",
    "Featured Video Play Icon",
    "Social Media Feed Footer",
    "Product Feature List Item",
    "Call to Action Link Text",
    "Footer Newsletter Submit",
    "Navigation Menu Item Link",
    "Featured Image Carousel Nav",
    "Testimonial Quote Text",
    "Blog Post Category List",
    "Company Phone Number",
    "Search Input Placeholder",
    "Product Feature Description",
    "Hero Section with Image",
    "Navigation Menu Toggle Text",
    "Featured Video Controls Bar",
    "Customer Review Profile",
    "Social Media Icon Set",
    "Footer Newsletter Input",
    "Product Feature Icon List",
    "Call to Action Button Text",
    "Featured Image Lightbox Image",
    "Testimonial Author Name",
    "Blog Post Author Info",
    "Navigation Menu Submenu",
    "Featured Video Poster Image",
    "Company Email Address",
    "Search Result Item Link",
    "Product Feature List Title",
    "Call to Action Link URL",
    "Footer Social Media Icon",
    "Hero Section with Form",
    "Navigation Menu Toggle Icon",
    "Featured Image Zoom Button",
    "Testimonial Quote Mark",
    "Blog Post Category Name",
    "Social Media Feed Header",
    "Product Description Textarea",
    "Navigation Breadcrumb Trail",
    "Featured Video Play Button",
    "Company Address Map",
    "Search Result Item Title",
    "Product Feature Icon Size",
    "Call to Action Button Color",
    "Featured Image Lightbox Close",
    "Testimonial Author Profile",
    "Footer Copyright Info",
    "Hero Section with Text",
    "Navigation Menu Submenu",
    "Featured Video Play Icon",
    "Social Media Feed Footer",
    "Product Feature List Item",
    "Call to Action Link Text",
    "Footer Newsletter Submit",
    "Navigation Menu Item Link",
    "Featured Image Carousel Nav",
    "Testimonial Quote Text",
    "Blog Post Category List",
    "Company Phone Number",
    "Search Input Placeholder",
    "Product Feature Description",
    "Hero Section with Image",
    "Navigation Menu Toggle Text",
    "Featured Video Controls Bar",
    "Customer Review Profile",
    "Social Media Icon Set",
    "Footer Newsletter Input",
    "Product Feature Icon List",
    "Call to Action Button Text",
    "Featured Image Lightbox Image",
    "Testimonial Author Name",
    "Blog Post Author Info",
    "Navigation Menu Submenu",
    "Featured Video Poster Image",
    "Company Email Address",
    "Search Result Item Link",
    "Product Feature List Title",
    "Call to Action Link URL",
    "Footer Social Media Icon",
    "Hero Section with Form",
    "Navigation Menu Toggle Icon",
    "Featured Image Zoom Button",
    "Testimonial Quote Mark",
    "Blog Post Category Name",
    "Social Media Feed Header",
    "Product Description Textarea",
    "Navigation Breadcrumb Trail",
    "Featured Video Play Button",
    "Company Address Map",
    "Search Result Item Title",
    "Product Feature Icon Size",
    "Call to Action Button Color",
    "Featured Image Lightbox Close",
    "Testimonial Author Profile",
    "Footer Copyright Info",
    "Hero Section with Text",
    "Navigation Menu Submenu",
    "Featured Video Play Icon",
    "Social Media Feed Footer",
    "Product Feature List Item",
    "Call to Action Link Text",
    "Footer Newsletter Submit",
    "Navigation Menu Item Link",
    "Featured Image Carousel Nav",
    "Testimonial Quote Text",
    "Blog Post Category List",
    "Company Phone Number",
    "Search Input Placeholder",
    "Product Feature Description",
    "Hero Section with Image",
    "Navigation Menu Toggle Text",
    "Featured Video Controls Bar",
    "Customer Review Profile",
    "Social Media Icon Set",
    "Footer Newsletter Input",
    "Product Feature Icon List",
    "Call to Action Button Text",
    "Featured Image Lightbox Image",
    "Testimonial Author Name",
    "Blog Post Author Info",
    "Navigation Menu Submenu",
    "Featured Video Poster Image",
    "Company Email Address",
    "Search Result Item Link",
    "Product Feature List Title",
    "Call to Action Link URL",
    "Footer Social Media Icon",
    "Hero Section with Form",
    "Navigation Menu Toggle Icon",
    "Featured Image Zoom Button",
    "Testimonial Quote Mark",
    "Blog Post Category Name",
    "Social Media Feed Header",
    "Product Description Textarea",
    "Navigation Breadcrumb Trail",
    "Featured Video Play Button",
    "Company Address Map",
    "Search Result Item Title",
    "Product Feature Icon Size",
    "Call to Action Button Color",
    "Featured Image Lightbox Close",
    "Testimonial Author Profile",
    "Footer Copyright Info",
    "Hero Section with Text",
    "Navigation Menu Submenu",
    "Featured Video Play Icon",
    "Social Media Feed Footer",
    "Product Feature List Item",
];
