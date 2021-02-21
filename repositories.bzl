"""A module for loading the third party repositories of cargo-raze"""

load("//third_party:third_party_repositories.bzl", "third_party_repositories")

def cargo_raze_repositories():
    third_party_repositories()
