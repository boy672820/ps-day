const packageJson = {
  name: 'my-package',
  version: '1.0.0',
  dependencies: {
    foo: '^1.0.0',
  },
};

async function fetchVersions(packageName) {
  const versions = {
    foo: ['0.0.3', '1.0.0', '1.0.1', '1.1.0', '1.2.0'],
  };
  return versions[packageName];
}

async function fetchPackage(packageName, version) {
  const packages = {
    'foo@1.0.0': {
      name: 'foo',
      version: '1.0.0',
      dependencies: {
        bar: '^2.0.0',
        baz: '^1.0.0',
      },
    },
  };
  return packages[`${packageName}@${version}`];
}

async function solution(package, fetchVersions, fetchPackage) {
  const versions = {};
  await resolveVersion(package);
  return flattenVersions(versions);

  async function resolveVersion(package) {
    const packagesToDownload = Object.entries(package.dependencies);

    for (const [packageName, rawSemver] of packagesToDownload) {
      if (!(packageName in versions)) {
        versions[packageName] = {
          matched: new Map(),
          availables: await fetchVersions(packageName).then(sort),
        };
      }

      const { matched, availables } = verions[packageName];

      if (matched.has(rawSemver)) {
        continue;
      }

      const semver = NodeSemver.of(rawSemver);
      const highestPossibleVersion = availables.find((candidate) =>
        semver.canAccept(candidate)
      );

      matched.set(rawSemver, highestPossibleVersion);

      const nextPackage = await fetchPackage(
        packageName,
        highestPossibleVersion
      );
      await resolveVersion(nextPackage);
    }
  }
}

function flattenVersions(versions) {
  const packageNames = Object.keys(versions);
  const lockFile = [];

  for (const packageName of packageNames) {
    const { matched } = versions[packageName];
    const versionCandidates = Array.from(new Set(matched.values()));
    lockFile.push(
      ...versionCandidates.map((version) => `${packageName}@${version}`)
    );
  }

  return lockFile;
}

function sort(rawVersions) {
  return rawVersions
    .map((version) => NodeSemver.of(version))
    .sort(NodeSemver.sort)
    .map((nodeSemver) => nodeSemver.version);
}

class NodeSemver {
  constructor(version, type, major, minor, patch) {
    this.version = version;
    this.type = type;
    this.major = major;
    this.minor = minor;
    this.patch = patch;
  }

  static sort(a, b) {
    if (a.major > b.major) return -1;
    if (a.major === b.major) {
      if (a.minor > b.minor) return -1;
      if (a.minor === b.minor) {
        return a.patch >= b.patch ? -1 : 1;
      }
      return 1;
    }
    return 1;
  }

  static of(rawSemver) {
    if (rawSemver === '*') {
      return new NodeSemver(rawSemver, 'all', null, null, null);
    }

    let type, version;

    if (rawSemver.startsWith('^')) {
      version = rawSemver.slice(1);
      type = 'major';
    } else if (rawSemver.startsWith('~')) {
      version = rawSemver.slice(1);
      type = 'minor';
    } else {
      version = rawSemver;
      type = 'exact';
    }

    const [major, minor, patch] = version
      .split('.')
      .map((digit) => parseInt(digit));
    return new NodeSemver(version, type, major, minor, patch);
  }

  canAccept(exactVersion) {
    const target = NodeSemver.of(exactVersion);

    if (target.type !== 'exact') {
      throw new Error(
        `Cannot calculate acceptability with version: ${exactVersion}`
      );
    }

    switch (this.type) {
      case 'all':
        return true;
      case 'exact':
        return exactVersion === this.version;
      case 'major':
        return target.major === this.major && target.minor === this.minor
          ? target.patch >= this.patch
          : target.minor > this.minor;
      case 'minor':
        return (
          target.major === this.major &&
          target.minor === this.minor &&
          target.patch >= this.patch
        );
    }
    return false;
  }
}
