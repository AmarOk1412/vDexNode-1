# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.10

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/sylvain/vDexNode

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/sylvain/vDexNode

# Include any dependencies generated for this target.
include tools/CMakeFiles/dhtnode.dir/depend.make

# Include the progress variables for this target.
include tools/CMakeFiles/dhtnode.dir/progress.make

# Include the compile flags for this target's objects.
include tools/CMakeFiles/dhtnode.dir/flags.make

tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o: tools/CMakeFiles/dhtnode.dir/flags.make
tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o: tools/dhtnode.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/sylvain/vDexNode/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o"
	cd /home/sylvain/vDexNode/tools && /usr/bin/c++  $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/dhtnode.dir/dhtnode.cpp.o -c /home/sylvain/vDexNode/tools/dhtnode.cpp

tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/dhtnode.dir/dhtnode.cpp.i"
	cd /home/sylvain/vDexNode/tools && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/sylvain/vDexNode/tools/dhtnode.cpp > CMakeFiles/dhtnode.dir/dhtnode.cpp.i

tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/dhtnode.dir/dhtnode.cpp.s"
	cd /home/sylvain/vDexNode/tools && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/sylvain/vDexNode/tools/dhtnode.cpp -o CMakeFiles/dhtnode.dir/dhtnode.cpp.s

tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.requires:

.PHONY : tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.requires

tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.provides: tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.requires
	$(MAKE) -f tools/CMakeFiles/dhtnode.dir/build.make tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.provides.build
.PHONY : tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.provides

tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.provides.build: tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o


# Object files for target dhtnode
dhtnode_OBJECTS = \
"CMakeFiles/dhtnode.dir/dhtnode.cpp.o"

# External object files for target dhtnode
dhtnode_EXTERNAL_OBJECTS =

tools/dhtnode: tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o
tools/dhtnode: tools/CMakeFiles/dhtnode.dir/build.make
tools/dhtnode: libopendht.so.1.10.0
tools/dhtnode: tools/CMakeFiles/dhtnode.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/sylvain/vDexNode/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable dhtnode"
	cd /home/sylvain/vDexNode/tools && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/dhtnode.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
tools/CMakeFiles/dhtnode.dir/build: tools/dhtnode

.PHONY : tools/CMakeFiles/dhtnode.dir/build

tools/CMakeFiles/dhtnode.dir/requires: tools/CMakeFiles/dhtnode.dir/dhtnode.cpp.o.requires

.PHONY : tools/CMakeFiles/dhtnode.dir/requires

tools/CMakeFiles/dhtnode.dir/clean:
	cd /home/sylvain/vDexNode/tools && $(CMAKE_COMMAND) -P CMakeFiles/dhtnode.dir/cmake_clean.cmake
.PHONY : tools/CMakeFiles/dhtnode.dir/clean

tools/CMakeFiles/dhtnode.dir/depend:
	cd /home/sylvain/vDexNode && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/sylvain/vDexNode /home/sylvain/vDexNode/tools /home/sylvain/vDexNode /home/sylvain/vDexNode/tools /home/sylvain/vDexNode/tools/CMakeFiles/dhtnode.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : tools/CMakeFiles/dhtnode.dir/depend

